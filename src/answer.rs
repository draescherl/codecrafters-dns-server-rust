use crate::dns_types::{DNSClass, DNSType};

#[derive(Debug)]
pub struct DNSAnswer {
    pub name: Vec<String>,
    pub answer_type: DNSType,
    pub class: DNSClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Vec<u8>,
}

impl DNSAnswer {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        for label in &self.name {
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
}
