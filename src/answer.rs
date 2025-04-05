use crate::dns_types::{DNSClass, DNSType};
use crate::name::Name;

#[derive(Clone, Debug)]
pub struct DNSAnswer {
    pub name: Name,
    pub answer_type: DNSType,
    pub class: DNSClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Vec<u8>,
}

impl DNSAnswer {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        for label in self.name.iter() {
            output.push(label.len() as u8);
            output.extend_from_slice(&label.as_bytes());
        }
        output.push(0); // Add null byte at the end of the labels

        output.extend(self.answer_type.encode());
        output.extend(self.class.encode());
        output.extend(self.ttl.to_be_bytes());
        output.extend(self.rd_length.to_be_bytes());
        output.extend(self.r_data.iter());

        output
    }

    pub fn parse(buffer: &[u8], current_index: usize) -> (Self, usize) {
        let (name, offset) = Name::parse(buffer, current_index);
        let answer_type = DNSType::parse([buffer[offset], buffer[offset + 1]]).unwrap();
        let class = DNSClass::parse([buffer[offset + 2], buffer[offset + 3]]).unwrap();

        let mut ttl = 0u32;
        for i in 4..8 {
            ttl = ttl << 8 | (buffer[offset + i] as u32);
        }

        let rd_length = (buffer[offset + 8] as u16) << 8 | buffer[offset + 9] as u16;
        let mut r_data: Vec<u8> = vec![];
        for i in 0..rd_length {
            r_data.push(buffer[offset + 10 + i as usize]);
        }

        (
            Self {
                name,
                answer_type,
                class,
                ttl,
                rd_length,
                r_data,
            },
            offset + 10 + rd_length as usize,
        )
    }
}
