use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl DNSHeader {
    pub fn new(
        id: u16,
        qr: bool,
        opcode: u8,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: u8,
        rcode: u8,
        qd_count: u16,
        an_count: u16,
        ns_count: u16,
        ar_count: u16,
    ) -> DNSHeader {
        let mut header = DNSHeader {
            id,
            flags: 0,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        };

        header.set_qr_flag(qr);
        header.set_opcode_flag(opcode);
        header.set_aa_flag(aa);
        header.set_tc_flag(tc);
        header.set_rd_flag(rd);
        header.set_ra_flag(ra);
        header.set_z_flag(z);
        header.set_rcode_flag(rcode);

        header
    }

    pub fn parse(input: &[u8]) -> DNSHeader {
        let id = (input[0] as u16) << 8 | (input[1] as u16);
        let qr = input[2] >> 7 != 0;
        let opcode = (input[2] & 0x78) >> 3;
        let aa = (input[2] & 0x04) >> 2 != 0;
        let tc = (input[2] & 0x02) >> 1 != 0;
        let rd = (input[2] & 0x01) != 0;

        let ra = (input[3] & 0x80) >> 7 != 0;
        let z = (input[3] & 0x70) >> 4;
        let rcode = input[3] & 0x0f;

        let qdcount = (input[4] as u16) << 8 | (input[5] as u16);
        let ancount = (input[6] as u16) << 8 | (input[7] as u16);
        let nscount = (input[8] as u16) << 8 | (input[9] as u16);
        let arcount = (input[10] as u16) << 8 | (input[11] as u16);

        let header = DNSHeader::new(
            id, qr, opcode, aa, tc, rd, ra, z, rcode, qdcount, ancount, nscount, arcount,
        );

        //println!("{}", header);
        header
    }

    pub fn encode(&self) -> [u8; 12] {
        let mut output = [0u8; 12];

        output[0..2].copy_from_slice(&self.id.to_be_bytes());
        output[2..4].copy_from_slice(&self.flags.to_be_bytes());
        output[4..6].copy_from_slice(&self.qd_count.to_be_bytes());
        output[6..8].copy_from_slice(&self.an_count.to_be_bytes());
        output[8..10].copy_from_slice(&self.ns_count.to_be_bytes());
        output[10..12].copy_from_slice(&self.ar_count.to_be_bytes());

        output
    }

    pub fn get_bit_at_pos(&self, pos: u32) -> bool {
        let mask = 2u16.pow(pos);
        self.flags & mask == mask
    }
    fn set_bit_at_pos(&mut self, pos: u32, value: bool) {
        self.flags = if value {
            self.flags | 2u16.pow(pos)
        } else {
            self.flags & (0xffff - 2u16.pow(pos))
        };
    }

    pub fn get_qr_flag(&self) -> bool {
        self.get_bit_at_pos(15)
    }
    pub fn set_qr_flag(&mut self, value: bool) {
        self.set_bit_at_pos(15, value);
    }

    pub fn get_opcode_flag(&self) -> u8 {
        ((self.flags & 0x7800) >> 11) as u8
    }
    fn set_opcode_flag(&mut self, value: u8) {
        if value >= 2u8.pow(4) {
            panic!("Opcode should be on 4 bits.");
        }
        self.flags = (self.flags & 0x87ff) | (value as u16) << 11;
    }

    pub fn get_aa_flag(&self) -> bool {
        self.get_bit_at_pos(10)
    }
    fn set_aa_flag(&mut self, value: bool) {
        self.set_bit_at_pos(10, value);
    }

    pub fn get_tc_flag(&self) -> bool {
        self.get_bit_at_pos(9)
    }
    fn set_tc_flag(&mut self, value: bool) {
        self.set_bit_at_pos(9, value);
    }

    pub fn get_rd_flag(&self) -> bool {
        self.get_bit_at_pos(8)
    }
    fn set_rd_flag(&mut self, value: bool) {
        self.set_bit_at_pos(8, value);
    }

    pub fn get_ra_flag(&self) -> bool {
        self.get_bit_at_pos(7)
    }
    fn set_ra_flag(&mut self, value: bool) {
        self.set_bit_at_pos(7, value);
    }

    pub fn get_z_flag(&self) -> u8 {
        ((self.flags & 0x0070) >> 4) as u8
    }
    fn set_z_flag(&mut self, value: u8) {
        if value >= 2u8.pow(3) {
            panic!("Opcode should be on 3 bits.");
        }
        self.flags = (self.flags & 0xff8f) | (value as u16) << 4;
    }

    pub fn get_rcode_flag(&self) -> u8 {
        (self.flags & 0x000f) as u8
    }
    fn set_rcode_flag(&mut self, value: u8) {
        if value >= 2u8.pow(4) {
            panic!("Opcode should be on 4 bits.");
        }
        self.flags = (self.flags & 0xfff0) | (value as u16);
    }
}

impl Display for DNSHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let encoded = self.encode();
        writeln!(f, "==== DNS Header ====")?;

        writeln!(f, "id: {}", self.id)?;

        // Flags
        writeln!(f, "qr: {}", self.get_qr_flag())?;
        writeln!(f, "opcode: {}", self.get_opcode_flag())?;
        writeln!(f, "aa: {}", self.get_aa_flag())?;
        writeln!(f, "tc: {}", self.get_tc_flag())?;
        writeln!(f, "rd: {}", self.get_rd_flag())?;
        writeln!(f, "ra: {}", self.get_ra_flag())?;
        writeln!(f, "z: {}", self.get_z_flag())?;
        writeln!(f, "rcode: {}", self.get_rcode_flag())?;

        writeln!(f, "qd_count: {}", self.qd_count)?;
        writeln!(f, "an_count: {}", self.an_count)?;
        writeln!(f, "ns_count: {}", self.ns_count)?;
        writeln!(f, "ar_count: {}", self.ar_count)?;

        for byte in encoded {
            writeln!(f, "{:#010b}", byte)?;
        }
        Ok(())
    }
}
