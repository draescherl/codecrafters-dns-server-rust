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
}
