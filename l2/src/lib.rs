pub mod model_metrics;
pub mod model;

use model::*;
use model_metrics::*;
use Bit::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    fn other(&self) -> Self {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero,
        }
    }
    fn to_usize(self) -> usize {
        match self {
            Zero => 0,
            One => 1,
        }
    }
}

impl Default for Bit {
    fn default() -> Self {
        Zero
    }
}

#[derive(Debug)]
pub struct BitVec {
    vec: Vec<u8>,
    inside_idx: usize,
    outside_idx: usize,
    inside_itr_couner: usize,
    outside_itr_couner: usize,
}

impl Iterator for BitVec {
    type Item = Bit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inside_itr_couner == 8 {
            self.inside_itr_couner = 0;
            self.outside_itr_couner += 1;
        }
        if self.outside_idx < self.outside_itr_couner {
            self.inside_itr_couner = 0;
            self.outside_itr_couner = 0;
            return None;
        }
        let output: Bit = if (self.vec[self.outside_itr_couner]
            & (0x80 >> self.inside_itr_couner))
            > 0
        {
            One
        } else {
            Zero
        };
        self.inside_itr_couner += 1;
        Some(output)
    }
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            vec: vec![0],
            inside_idx: 0,
            outside_idx: 0,
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }

    fn push(&mut self, bit: Bit) {
        if self.inside_idx == 8 {
            self.inside_idx = 0;
            self.outside_idx += 1;
            self.vec.push(0);
        }
        match bit {
            Bit::Zero => (),
            Bit::One => self.vec[self.outside_idx] ^= 0x80 >> self.inside_idx,
        }
        self.inside_idx += 1;
    }
}

pub fn encode_arithmetic(file: &Vec<u8>) -> BitVec {
    let mut high: usize = CODE_MAX;
    let mut low: usize = 0;
    let mut pending_bits = 0;
    let mut model = Model::new();
    let mut compresed = BitVec::new();
    for c in file {
        let c = if *c as i8 == -1 {
            256
        } else  {
            *c as usize
        };
        
        let range = (high - low) + 1;
        let p = model.prob(c);
        high = low + (range * p.0) / p.2;
        low += (range * p.1) / p.2;
        loop {
            if high < CODE_HALF {
                //println!("{:?}", pending_bits);
                output_bit_plus_pending(&mut compresed, Zero, pending_bits);
                //println!("{:?}", pending_bits);
            } else if low >= CODE_HALF {
                //println!("{:?}", pending_bits);
                output_bit_plus_pending(&mut compresed, One, pending_bits);
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
        if c == 256 {
            break;
        }
    }
    pending_bits += 1;
    if low < CODE_FIRST_QTR{
        output_bit_plus_pending(&mut compresed, One, pending_bits);
    } else {
        output_bit_plus_pending(&mut compresed, One, pending_bits);
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

pub fn decode(mut bitvec: BitVec) -> Vec<u8> {
    let mut high = CODE_MAX;
    let mut low = 0;
    let mut value = 0;
    for _ in 0..CODE_BITS {
        value <<= 1;
        value += bitvec.next().unwrap().to_usize();
    }
    let mut model  = Model::new();
    let mut decompressed: Vec<u8> = Vec::new();
    loop {
        let range = high - low + 1;
        let scaled_value = ((value - low + 1) * model.count() - 1 ) / range;
        let (p, c) = model.get_char(scaled_value).unwrap();
        if c  == 256 {
            break;
        }
        decompressed.push(c as u8);
        high = low + (range * p.1)/p.2 - 1;
        low = low + (range * p.0)/p.2;
        loop {
            if high < CODE_HALF {
                //do nothing, bit is a zero
            } else if low >= CODE_HALF {
                value -= CODE_HALF;  //subtract one half from all three code values
                low -= CODE_HALF;
                high -= CODE_HALF;
            } else if low >= CODE_FIRST_QTR && high < CODE_THIRD_QTR {
                value -= CODE_FIRST_QTR;
                low -= CODE_FIRST_QTR;
                high -= CODE_FIRST_QTR;
            } else {
                break;
            }
            low <<= 1;
            high <<= 1;
            high += 1;
            value <<= 1;
            //println!("{:?}, {:?}", bitvec.inside_idx, bitvec.inside_itr_couner);
            //println!("{:?}, {:?}", bitvec.outside_idx, bitvec.outside_itr_couner);
            value += bitvec.next().unwrap().to_usize();
        }
        println!("{:?}", &decompressed);
    }
    decompressed
}

#[cfg(test)]
mod tests {
    use super::Bit::{One, Zero};
    use super::*;
    #[test]
    fn encode_decode_test() {
        let file = std::fs::read("test_cases/test3.bin").unwrap();
        let encoded = encode_arithmetic(&file);
        println!("{:?}", &encoded);
        let decoded = decode(encoded);
        println!("{:?}", &decoded);
        //assert_eq!(file, decoded);
    }

    #[test]
    fn bitvec_test() {
        let mut vec = BitVec::new();
        vec.push(One);
        vec.push(Zero);
        vec.push(One);
        assert_eq!(vec.vec[0], 0xA0);
        assert_eq!(vec.next().unwrap(), One);
        assert_eq!(vec.next().unwrap(), Zero);
        assert_eq!(vec.next().unwrap(), One);
        //assert_eq!(vec.next(), None);
    }
}
