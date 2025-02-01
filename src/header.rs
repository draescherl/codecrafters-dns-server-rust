use std::fmt::{Display, Formatter};

pub struct DnsHeader {
    pub id: u16,
    pub qr: bool,
    pub opcode: u8,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn encode(&self) -> Vec<u8> {
        let mut output = [0u8; 12];
        output[0] = (self.id >> 8) as u8;
        output[1] = self.id as u8;

        let tmp = ((self.qr as u8) << 7)
            + (self.opcode << 3)
            + ((self.aa as u8) << 2)
            + ((self.tc as u8) << 1)
            + (self.rd as u8);
        output[2] = tmp;

        let tmp = (self.ra as u8) << 7 + (self.z << 4) + self.rcode;
        output[3] = tmp;

        output[4] = (self.qdcount >> 8) as u8;
        output[5] = self.qdcount as u8;

        output[6] = (self.ancount >> 8) as u8;
        output[7] = self.ancount as u8;

        output[8] = (self.nscount >> 8) as u8;
        output[9] = self.nscount as u8;

        output[10] = (self.arcount >> 8) as u8;
        output[11] = self.arcount as u8;

        output.to_vec()
    }
}

impl Display for DnsHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let encoded = self.encode();
        write!(f, "==== DNS Header ====\n")?;
        for byte in encoded {
            write!(f, "{:#010b}\n", byte)?;
        }
        Ok(())
    }
}
