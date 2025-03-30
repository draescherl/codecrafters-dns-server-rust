use crate::answer::DNSAnswer;
use crate::dns_types::{DNSClass, DNSType};

#[derive(Clone, Debug)]
pub struct DNSQuestion {
    pub name: Vec<String>,
    pub question_type: DNSType,
    pub class: DNSClass,
}

impl DNSQuestion {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        for label in &self.name {
            output.push(label.len() as u8);
            output.extend_from_slice(&label.as_bytes());
        }
        output.push(0); // Add null byte at the end of the labels

        output.extend(self.question_type.encode());
        output.extend(self.class.encode());

        output
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
                let pointer = (((buffer[i] & 0b00111111) as u16) << 8 | (buffer[i+1] as u16)) as usize;
                let (mut tmp, _) = DNSQuestion::extract_name(buffer, pointer);
                // the second byte of the pointer will be skipped by the last +1 (in the return)
                i += 1;
                name.append(&mut tmp);
                break
            } else {
                i += 1;
                let label: String = buffer[i..i + length]
                    .iter()
                    .map(|&c| c as char)
                    .collect();
                i += length;
                name.push(label);
            };
        }

        (name, i + 1)
    }

    pub fn parse(buffer: &[u8], current_index: usize) -> (DNSQuestion, usize) {
        let (name, offset) = DNSQuestion::extract_name(buffer, current_index);
        let question_type = DNSType::parse([buffer[offset], buffer[offset + 1]]).unwrap();
        let class = DNSClass::parse([buffer[offset + 2], buffer[offset + 3]]).unwrap();
        (
            DNSQuestion {
                name,
                question_type,
                class,
            },
            offset + 4,
        )
    }

    pub fn reply(&self) -> DNSAnswer {
        DNSAnswer {
            name: self.name.clone(),
            answer_type: self.question_type.clone(),
            class: self.class.clone(),
            ttl: 60,
            rd_length: 4,
            r_data: vec![8, 8, 8, 8],
        }
    }
}
