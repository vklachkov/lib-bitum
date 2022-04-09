use crate::bit_pos::BitPosition;
use crate::ser::*;
use crate::util::*;

macro_rules! gen {
    ($t:ty) => {
        impl BitumSerializeOwned for $t {
            fn serialize_into<const N: usize>(&self, data: &mut [u8; N], pos: BitPosition) -> BitPosition {
                let mut pos = pos;
        
                let self_bytes = self.to_le_bytes();
        
                if pos.is_round() {
                    for b in self_bytes {
                        data[pos.byte] = b;
                        pos = pos.inc_bytes(1);
                    }
        
                    pos
                } else {
                    for b in self_bytes {
                        let bit = 8 - pos.bit;

                        let high = extract_high_bits(b, bit);
                        let low = extract_low_bits(b, bit);
        
                        data[pos.byte + 0] |= low << pos.bit;
                        data[pos.byte + 1] |= high;
        
                        pos = pos.inc_bytes(1);
                    }
        
                    pos
                }
            }
        }
        
        impl BitumSerializeSomeBitsOwned for $t {
            fn serialize_bits_into<const N: usize>(
                &self,
                data: &mut [u8; N],
                count: usize,
                pos: BitPosition,
            ) -> BitPosition {
                let mask = (1 as $t << count) - 1;
                let discarded_bits_count = <$t>::BITS as usize - count;
                (self & mask).serialize_into(data, pos).sub_bits(discarded_bits_count)
            }
        }
    };
}

gen!(u8);
gen!(u16);
gen!(u32);
gen!(u64);
gen!(u128);
