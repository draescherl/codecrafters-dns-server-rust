use crate::answer::DNSAnswer;
use crate::{header::DNSHeader, question::DNSQuestion};

#[derive(Debug)]
pub struct DNSMessage {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSAnswer>,
}

impl DNSMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        let mut tmp = self.header.encode();
        output.extend_from_slice(&mut tmp);

        for question in &self.questions {
            let mut tmp = question.encode();
            output.append(&mut tmp);
        }

        for answer in &self.answers {
            let mut tmp = answer.encode();
            output.append(&mut tmp);
        }

        output
    }
}
