#[derive(Debug, Clone)]
pub struct Name {
    labels: Vec<String>,
}

pub struct NameIterator<'a> {
    name: &'a Name,
    index: usize,
}

impl Iterator for NameIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.name.labels.len() {
            let element = &self.name.labels[self.index];
            self.index += 1;
            Some(element.clone())
        } else {
            None
        }
    }
}

impl Name {
    pub fn iter(&self) -> NameIterator {
        NameIterator {
            name: &self,
            index: 0,
        }
    }

    fn extract_name(buffer: &[u8], start_from: usize) -> (Vec<String>, usize) {
        let mut i = start_from;
        let mut name: Vec<String> = vec![];
        while i < buffer.len() && buffer[i] != 0 {
            let length = buffer[i] as usize;

            // 192 = 0b1100_0000. This is used to signal the presence of a pointer.
            // Pointers are used to compress labels which were already encountered in a previous
            // question
            if length >= 192 {
                let mask = 0b00111111;
                let pointer = (((buffer[i] & mask) as u16) << 8 | (buffer[i + 1] as u16)) as usize;
                let (mut tmp, _) = Self::extract_name(buffer, pointer);
                // the second byte of the pointer will be skipped by the last +1 (in the return)
                i += 1;
                name.append(&mut tmp);
                break;
            } else {
                i += 1;
                let label: String = buffer[i..i + length].iter().map(|&c| c as char).collect();
                i += length;
                name.push(label);
            };
        }

        (name, i + 1)
    }

    pub fn parse(buffer: &[u8], start_from: usize) -> (Self, usize) {
        let (labels, i) = Self::extract_name(buffer, start_from);
        (Name { labels }, i)
    }
}
