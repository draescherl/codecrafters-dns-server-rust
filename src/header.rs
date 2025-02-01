use std::fmt::{Display, Formatter};

#[derive(Debug)]
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

    fn get_bit_at_pos(&self, pos: u32) -> bool {
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

    fn get_qr_flag(&self) -> bool {
        self.get_bit_at_pos(15)
    }
    fn set_qr_flag(&mut self, value: bool) {
        self.set_bit_at_pos(15, value);
    }

    fn get_opcode_flag(&self) -> u8 {
        ((self.flags & 0x7800) >> 11) as u8
    }
    fn set_opcode_flag(&mut self, value: u8) {
        if value >= 2u8.pow(4) {
            panic!("Opcode should be on 4 bits.");
        }
        self.flags = (self.flags & 0x87ff) | (value as u16) << 11;
    }

    fn get_aa_flag(&self) -> bool {
        self.get_bit_at_pos(10)
    }
    fn set_aa_flag(&mut self, value: bool) {
        self.set_bit_at_pos(10, value);
    }

    fn get_tc_flag(&self) -> bool {
        self.get_bit_at_pos(9)
    }
    fn set_tc_flag(&mut self, value: bool) {
        self.set_bit_at_pos(9, value);
    }

    fn get_rd_flag(&self) -> bool {
        self.get_bit_at_pos(8)
    }
    fn set_rd_flag(&mut self, value: bool) {
        self.set_bit_at_pos(8, value);
    }

    fn get_ra_flag(&self) -> bool {
        self.get_bit_at_pos(7)
    }
    fn set_ra_flag(&mut self, value: bool) {
        self.set_bit_at_pos(7, value);
    }

    fn get_z_flag(&self) -> u8 {
        ((self.flags & 0x0070) >> 4) as u8
    }
    fn set_z_flag(&mut self, value: u8) {
        if value >= 2u8.pow(3) {
            panic!("Opcode should be on 3 bits.");
        }
        self.flags = (self.flags & 0xff8f) | (value as u16) << 4;
    }

    fn get_rcode_flag(&self) -> u8 {
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
