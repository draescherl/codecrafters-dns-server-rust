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

    pub fn split_questions(&self) -> Vec<Self> {
        let mut items: Vec<Self> = vec![];
        let header = DNSHeader {
            qd_count: 1,
            an_count: 0,
            ..self.header
        };
        for question in &self.questions {
            let message = Self {
                header: header.clone(),
                questions: vec![question.clone()],
                answers: vec![],
            };
            items.push(message);
        }
        items
    }

    pub fn merge(reference_header: DNSHeader, questions: Vec<DNSQuestion>, answers: Vec<DNSAnswer>) -> Self {
        let header = DNSHeader {
            qd_count: questions.len() as u16,
            an_count: answers.len() as u16,
            ..reference_header
        };
        Self {
            header,
            questions,
            answers,
        }
    }

    pub fn parse(input: &[u8]) -> Self {
        let header = DNSHeader::parse(&input[0..12]);
        let num_questions = header.qd_count;
        let mut questions: Vec<DNSQuestion> = vec![];
        let mut i = 12;
        for _ in 0..num_questions {
            let (question, new_index) = DNSQuestion::parse(input, i);
            i = new_index;
            questions.push(question);
        }

        let num_answers = header.an_count;
        let mut answers: Vec<DNSAnswer> = vec![];
        for _ in 0..num_answers {
            let (answer, new_index) = DNSAnswer::parse(input, i);
            i = new_index;
            answers.push(answer);
        }

        Self {
            header,
            questions,
            answers,
        }
    }

    pub fn reply(&self) -> Self {
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

        Self {
            header: response_header,
            questions: self.questions.clone(),
            answers,
        }
    }
}
