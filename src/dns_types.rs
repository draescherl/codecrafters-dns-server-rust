#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DNSType {
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
impl DNSType {
    pub fn encode(&self) -> [u8; 2] {
        match self {
            DNSType::A => 1u16.to_be_bytes(),
            DNSType::NS => 2u16.to_be_bytes(),
            DNSType::CNAME => 5u16.to_be_bytes(),
            DNSType::SOA => 6u16.to_be_bytes(),
            DNSType::WKS => 11u16.to_be_bytes(),
            DNSType::PTR => 12u16.to_be_bytes(),
            DNSType::HINFO => 13u16.to_be_bytes(),
            DNSType::MINFO => 14u16.to_be_bytes(),
            DNSType::MX => 15u16.to_be_bytes(),
            DNSType::TXT => 16u16.to_be_bytes(),
        }
    }

    pub fn parse(input: [u8; 2]) -> Option<DNSType> {
        let combined = u16::from_be_bytes([input[0], input[1]]);
        match combined {
            1 => Some(DNSType::A),
            2 => Some(DNSType::NS),
            5 => Some(DNSType::CNAME),
            6 => Some(DNSType::SOA),
            11 => Some(DNSType::WKS),
            12 => Some(DNSType::PTR),
            13 => Some(DNSType::HINFO),
            14 => Some(DNSType::MINFO),
            15 => Some(DNSType::MX),
            16 => Some(DNSType::TXT),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DNSClass {
    IN,
    CH,
    HS,
}
impl DNSClass {
    pub fn encode(&self) -> [u8; 2] {
        match self {
            DNSClass::IN => 1u16.to_be_bytes(),
            DNSClass::CH => 3u16.to_be_bytes(),
            DNSClass::HS => 4u16.to_be_bytes(),
        }
    }

    pub fn parse(input: [u8; 2]) -> Option<DNSClass> {
        let combined = u16::from_be_bytes([input[0], input[1]]);
        match combined {
            1 => Some(DNSClass::IN),
            3 => Some(DNSClass::CH),
            4 => Some(DNSClass::HS),
            _ => None,
        }
    }
}
