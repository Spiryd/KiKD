const CODE_BITS: usize = 8;
const CODE_MAX: usize = (1 << CODE_BITS) - 1;
const CODE_FIRST_QTR: usize = CODE_MAX / 4 + 1;
const CODE_HALF: usize = 2 * CODE_FIRST_QTR;
const CODE_THIRD_QTR: usize = 3 * CODE_FIRST_QTR;

#[derive(Debug, Clone, Copy)]
enum Bit {
    Zero,
    One
}

impl Bit {
    fn other(&self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
}

#[derive(Debug)]
pub struct BitVec {
    vec: Vec<u64>,
    inside_idx: usize,
    outside_idx: usize
}

impl BitVec {
    fn new() -> BitVec {
        BitVec{vec: vec![0], inside_idx: 0, outside_idx: 0}    
    }
    fn push(&mut self, bit: Bit) {
        if self.inside_idx == 64 {
            self.inside_idx = 0;
            self.outside_idx += 1;
            self.vec.push(0);
        }
        match bit {
            Bit::Zero => (),
            Bit::One => {
                self.vec[self.outside_idx] ^= 0x8000_0000_0000_0000>> self.inside_idx
            },
        }
        self.inside_idx += 1;
    }
}

pub fn encode_arithmetic(file: &Vec<u8>) -> BitVec {
    let mut high: usize = CODE_MAX;
    let mut low: usize = 0;
    let mut pending_bits = 0;

    let mut compresed = BitVec::new();
    for c in file {
        
        let range = (high - low) + 1;
        high = low + (range * *c as usize) / 256;
        low = low + (range * (*c as usize + 1) ) / 256;
        loop {
            if high < CODE_HALF {
                //println!("{:?}", pending_bits);
                output_bit_plus_pending(&mut compresed, Bit::Zero, pending_bits);
                //println!("{:?}", pending_bits);
            } else if low >= CODE_HALF  {
                //println!("{:?}", pending_bits);
                output_bit_plus_pending(&mut compresed, Bit::One, pending_bits);
                //println!("{:?}", pending_bits);
            } else if low >= CODE_FIRST_QTR && high < CODE_THIRD_QTR {
                pending_bits += 1;
                low -= CODE_FIRST_QTR;
                high -= CODE_FIRST_QTR;
            } else {
                break;
            }
            high <<= 1;
            high += 1;
            low <<= 1;
            high &= CODE_MAX;
            low &= CODE_MAX;
        }
    }
    compresed
}

fn output_bit_plus_pending(bitvec: &mut BitVec, bit: Bit, mut pending_bits: usize) {
    bitvec.push(bit);
    while pending_bits > 0 {
        bitvec.push(bit.other());
        pending_bits -= 1;
    }
}

pub fn decode_arithmetic(bitvec: BitVec) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Bit::{Zero, One};
    #[test]
    fn encode_decode_test() {
        let file = std::fs::read("test_cases/test2.bin").unwrap();
        let encoded = encode_arithmetic(&file);
        println!("{:?}", encoded);
        //let decoded = decode_arithmetic();
        //assert_eq!(file, decoded);
    }

    #[test]
    fn bitvec_test() {
        let mut vec = BitVec::new();
        vec.push(One);
        vec.push(Zero);
        vec.push(One);
        assert_eq!(vec.vec[0], 0xA000_0000_0000_0000);
    }
}
