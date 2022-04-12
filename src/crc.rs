pub trait BitumCrc8 {
    fn crc(&self, crc: u8, table: &[u8; 256]) -> u8;
}

pub trait BitumCrc16 {
    fn crc(&self, crc: u16, table: &[u16; 256]) -> u16;
}