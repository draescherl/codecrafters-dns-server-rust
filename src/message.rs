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

    pub fn parse(input: &[u8]) -> DNSMessage {
        let header = DNSHeader::parse(&input[0..12]);
        let num_questions = header.qd_count;
        let mut buffer = &input[12..];
        let mut questions: Vec<DNSQuestion> = vec![];
        for _ in 0..num_questions {
            let (question, consumed) = DNSQuestion::parse(buffer);
            buffer = consumed;
            questions.push(question);
        }

        DNSMessage {
            header,
            questions,
            answers: vec![],
        }
    }

    pub fn reply(&self) -> DNSMessage {
        let mut answers: Vec<DNSAnswer> = vec![];
        for question in &self.questions {
            answers.push(question.reply());
        }

        let rcode = if self.header.get_opcode_flag() == 0 {
            0
        } else {
            4
        };
        let response_header = DNSHeader::new(
            self.header.id,
            true,
            self.header.get_opcode_flag(),
            false,
            false,
            self.header.get_rd_flag(),
            false,
            0,
            rcode,
            self.header.qd_count,
            answers.len() as u16,
            0,
            0,
        );

        DNSMessage {
            header: response_header,
            questions: self.questions.clone(),
            answers,
        }
    }
}
