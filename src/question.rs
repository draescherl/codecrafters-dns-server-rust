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

    pub fn parse(buffer: &[u8]) -> (DNSQuestion, &[u8]) {
        let mut i = 0;
        let mut name: Vec<String> = Vec::new();
        while i < buffer.len() && buffer[i] != 0 {
            let length = buffer[i] as usize + 1;
            let s: String = buffer[i + 1..i + length]
                .iter()
                .map(|&c| c as char)
                .collect();
            name.push(s);
            i += length;
        }
        i += 1;
        let question_type = DNSType::parse([buffer[i], buffer[i + 1]]).unwrap();
        let class = DNSClass::parse([buffer[i + 2], buffer[i + 3]]).unwrap();
        (
            DNSQuestion {
                name,
                question_type,
                class,
            },
            &buffer[i + 3..],
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
