#[derive(Debug)]
#[allow(dead_code)]
pub enum QuestionType {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
}
impl QuestionType {
    pub fn encode(&self) -> [u8; 2] {
        match self {
            QuestionType::A => 1u16.to_be_bytes(),
            QuestionType::NS => 2u16.to_be_bytes(),
            QuestionType::CNAME => 5u16.to_be_bytes(),
            QuestionType::SOA => 6u16.to_be_bytes(),
            QuestionType::WKS => 11u16.to_be_bytes(),
            QuestionType::PTR => 12u16.to_be_bytes(),
            QuestionType::HINFO => 13u16.to_be_bytes(),
            QuestionType::MINFO => 14u16.to_be_bytes(),
            QuestionType::MX => 15u16.to_be_bytes(),
            QuestionType::TXT => 16u16.to_be_bytes(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum QuestionClass {
    IN,
    CH,
    HS,
}
impl QuestionClass {
    pub fn encode(&self) -> [u8; 2] {
        match self {
            QuestionClass::IN => 1u16.to_be_bytes(),
            QuestionClass::CH => 3u16.to_be_bytes(),
            QuestionClass::HS => 4u16.to_be_bytes(),
        }
    }
}

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: Vec<String>,
    pub qtype: QuestionType,
    pub class: QuestionClass,
}

impl DNSQuestion {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];

        for label in &self.name {
            output.push(label.len() as u8);
            output.extend_from_slice(&label.as_bytes());
        }
        output.push(0); // Add null byte at the end of the labels

        output.extend(self.qtype.encode());
        output.extend(self.class.encode());

        output
    }
}
