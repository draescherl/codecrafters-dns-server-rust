use crate::answer::DNSAnswer;
use crate::dns_types::{DNSClass, DNSType};
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
        let question = DNSQuestion {
            name: vec!["codecrafters".to_string(), "io".to_string()],
            question_type: DNSType::A,
            class: DNSClass::IN,
        };

        DNSMessage {
            header,
            questions: vec![question],
            answers: vec![],
        }
    }

    pub fn reply(&self) -> DNSMessage {
        let answer = DNSAnswer {
            name: vec!["codecrafters".to_string(), "io".to_string()],
            answer_type: DNSType::A,
            class: DNSClass::IN,
            ttl: 60,
            rd_length: 4,
            r_data: vec![8, 8, 8, 8],
        };
        let answers = vec![answer];

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
