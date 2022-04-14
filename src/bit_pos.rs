use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[inline]
    pub fn bits(&self) -> usize {
        self.byte * 8 + self.bit
    }

    #[inline]
    pub fn is_round(&self) -> bool {
        self.bit == 0
    }

    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    pub fn round_down(&self) -> Self {
        BitPosition::new(self.byte, 0)
    }

    #[inline]
    #[must_use]
    pub fn inc_bytes(&self, bytes: usize) -> Self {
        BitPosition::new(self.byte + bytes, self.bit)
    }

    #[inline]
    #[must_use]
    pub fn inc_bits(&self, bits: usize) -> Self {
        let bits = self.bit + bits;
        BitPosition::new(self.byte + (bits / 8), bits % 8)
    }

    #[inline]
    #[must_use]
    pub fn sub_bytes(&self, bytes: usize) -> Self {
        BitPosition::new(self.byte - bytes, self.bit)
    }

    #[inline]
    #[must_use]
    pub fn sub_bits(&self, bits: usize) -> Self {
        let result_bits = self.bits() - bits;
        BitPosition::new(result_bits / 8, result_bits % 8)
    }
}
