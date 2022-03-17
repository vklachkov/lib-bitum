#[derive(Debug)]
pub struct BitPosition {
    byte: usize,
    bit: usize,
}

impl BitPosition {
    pub fn new(byte: usize, bit: usize) -> Self {
        BitPosition { byte, bit }
    }

    pub fn bits(&self) -> usize {
        self.byte * 8 + self.bit
    }

    pub fn is_round(&self) -> bool {
        self.bit == 0
    }

    pub fn round_up(&self) -> Self {
        BitPosition::new(
            if self.bit != 0 {
                self.byte + 1
            } else {
                self.byte
            },
            0,
        )
    }

    pub fn round_down(&self) -> Self {
        BitPosition::new(self.byte, 0)
    }
}

pub trait BitumExtract {
    fn extract_high_bits(byte: u8, bit: usize) -> u8 {
        let lmask = !((1 << bit) - 1);
        let lvbits = byte & lmask;
        let lrbits = lvbits >> bit;

        lrbits
    }

    fn extract_low_bits(byte: u8, bit: usize) -> u8 {
        let hmask = (1 << bit) - 1;
        let hvbits = byte & hmask;
        let hrbits = hvbits;

        hrbits
    }

    fn extract<const N: usize>(data: &[u8; N], at: &BitPosition) -> Self;

    fn extract_bits<const N: usize>(data: &[u8; N], at: &BitPosition, count: usize) -> Self;
}

macro_rules! number_extract_gen {
    ($t:ty) => {
        impl BitumExtract for $t {
            fn extract<const N: usize>(data: &[u8; N], at: &BitPosition) -> Self {
                const BYTES: usize = (<$t>::BITS / 8) as usize;
        
                if at.is_round() {
                    let start_index = at.byte;
                    let end_index = start_index + BYTES as usize;
                    let bytes: [u8; BYTES] = data[start_index..end_index].try_into().unwrap();
                    <$t>::from_le_bytes(bytes)
                } else {
                    /*
                    Extract from non rounded positon

                    Example bytes:
                    00110001 10001110 01001101 01111001
        
                    Simple example. One byte
        
                    Position is 1.5:
                    00110001 10001110 01001101 01111001
                                  ^__ ____^
                    Steps:
                    1. Extract first part:
                      1a. Get byte at 1, 0b10001110
                      1b. Extract high bits from 5, 0b110
                    2. Extract second part:
                      2a. Get byte at 2, 0b01001101
                      2b. Extract low bits up to 5, 0b01001
                    3. Calculate second part offset:
                      For one byte, u8, BYTES were equal 1
                      (8 - 5) + ((BYTES - 1) * 8) = 8 - 5 + 0 * 8 = 3
                    4. Glue the byte:
                      (0b110) | (0b01001 << 3)
        
                    Hard example. Two byte
        
                    Position is 0.2:
                    00110001 10001110 01001101 01111001
                      ^_____ ________ _^
        
                    Steps:
                    1. Extract first part:
                      1a. Get byte at 0, 0b00110001
                      1b. Extract high bits from 2, 0b110001
                    2. Extract middle bytes:
                      For i in BYTES-1
                        - Extract byte at rounded up start position + i
                        - Get byte
                        - Glue with accumulator:
                          middle_bytes |= byte << (i * 8)
                    3. Extract second part:
                      3a. Get byte at start position plus BYTES bytes, 0b01001101
                      3b. Extract low bits up to 5, 0b01
                    4. Calculate middle part offset:
                      (8 - 2) = 6
                    5. Calculate second part offset:
                      For two bytes, u16, BYTES were equal 2
                      (8 - 2) + ((BYTES - 1) * 8) = 8 - 2 + 1 * 8 = 14
                    6. Glue the byte:
                      (0b110001) | (middle_bytes << 6) | (0b01 << 14)
                    */
        
                    let start_byte = data[at.byte];
                    let start_bits = Self::extract_high_bits(start_byte, at.bit) as $t;
        
                    let mut middle_bits: $t = 0;
                    for i in 0..BYTES - 1 {
                        let middle_byte = data[at.round_up().byte + i] as $t;
                        middle_bits |= middle_byte << (i * 8);
                    }

                    let finish_byte = data[at.byte + BYTES];
                    let finish_bits = Self::extract_low_bits(finish_byte, at.bit) as $t;
        
                    let middle_part_offset = (8 - at.bit);
                    let finish_part_offset = (8 - at.bit) + ((BYTES - 1) * 8);
        
                    //println!("Start = {:8b}", start_bits);
                    //println!("Middle = {:8b}", middle_bits);
                    //println!("Finish = {:8b}", finish_bits);

                    (start_bits << 0) | (middle_bits << middle_part_offset) | (finish_bits << finish_part_offset)
                }
            }
        
            fn extract_bits<const N: usize>(data: &[u8; N], at: &BitPosition, count: usize) -> Self {
              assert!(count > Self::BITS as usize);

              // TODO: Optimize, if bits significantly smaller than the bit in the type
              Self::extract(data, at) & ((1 << at.bit) - 1) 
            }
        }        
    };
}

number_extract_gen!(u8);
number_extract_gen!(u16);
number_extract_gen!(u32);
number_extract_gen!(u64);