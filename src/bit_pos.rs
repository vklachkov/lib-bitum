#[derive(Debug, Clone, Copy)]
pub struct BitPosition {
    pub byte: usize,
    pub bit: usize,
}

impl BitPosition {
    pub fn zero() -> Self {
        BitPosition { byte: 0, bit: 0 }
    }

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

    pub fn inc_bytes(&self, bytes: usize) -> Self {
        BitPosition::new(self.byte + bytes, self.bit)
    }

    pub fn inc_bits(&self, bits: usize) -> Self {
        let total_bits = self.bit + bits;
        BitPosition::new(self.byte + (total_bits / 8), total_bits % 8)
    }
}
