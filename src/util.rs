pub fn extract_high_bits(byte: u8, bit: usize) -> u8 {
    let lmask = !((1 << bit) - 1);
    let lvbits = byte & lmask;
    let lrbits = lvbits >> bit;

    lrbits
}

pub fn extract_low_bits(byte: u8, bit: usize) -> u8 {
    let hmask = (1 << bit) - 1;
    let hvbits = byte & hmask;
    let hrbits = hvbits;

    hrbits
}