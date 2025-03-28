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
}
