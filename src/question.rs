use crate::answer::DNSAnswer;
use crate::dns_types::{DNSClass, DNSType};
use crate::name::Name;

#[derive(Clone, Debug)]
pub struct DNSQuestion {
    pub name: Name,
    pub question_type: DNSType,
    pub class: DNSClass,
}

impl DNSQuestion {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        for label in self.name.iter() {
            output.push(label.len() as u8);
            output.extend_from_slice(&label.as_bytes());
        }
        output.push(0); // Add null byte at the end of the labels

        output.extend(self.question_type.encode());
        output.extend(self.class.encode());

        output
    }

    pub fn parse(buffer: &[u8], current_index: usize) -> (Self, usize) {
        let (name, offset) = Name::parse(buffer, current_index);
        let question_type = DNSType::parse([buffer[offset], buffer[offset + 1]]).unwrap();
        let class = DNSClass::parse([buffer[offset + 2], buffer[offset + 3]]).unwrap();
        (
            Self {
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
            ttl: 2048,
            rd_length: 4,
            r_data: vec![8, 9, 10, 11],
        }
    }
}
