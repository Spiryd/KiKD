enum Bit {
    Zero,
    One
}

#[derive(Debug)]
struct BitVec {
    vec: Vec<u64>,
    inside_idx: usize,
    outside_idx: usize
}

impl BitVec {
    fn new() -> BitVec {
        BitVec{vec: vec![0x0000_0000_0000_0000], inside_idx: 0, outside_idx: 0}    
    }
    fn push(&mut self, bit: Bit) {
        if self.inside_idx == 64 {
            self.inside_idx = 0;
            self.outside_idx += 1;
            self.vec.push(0u64);
        }
        match bit {
            Bit::Zero => (),
            Bit::One => {
                self.vec[self.outside_idx] ^= 0x8000_0000_0000_0000 >> self.inside_idx
            },
        }
        self.inside_idx += 1;
    }
}

pub fn encode_arithmetic(file: &Vec<u8>) {
    let mut high: u32 = 0xFFFFFFFF;
    let mut low: u32 = 0;
    for c in file {
        let range = high - low + 1;
        let p = prob(*c);
        high = low + (range * p.1)/p.2;
        low = low + (range * p.0)/p.2;
        loop {
            if high < 0x80000000 {
                
            } else if low >= 0x80000000 {
                
            } else {
                break;
            }
        }
    }
    todo!()
}



fn prob(c: u8) -> (u32, u32, u32) {
    (c as u32, c as u32 + 1, 256)
}

pub fn decode_arithmetic() -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Bit::{Zero, One};
    //#[test]
    fn encode_decode_test() {
        let file = std::fs::read("test_cases/pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt").unwrap();
        let encoded = encode_arithmetic(&file);
        let decoded = decode_arithmetic();
        assert_eq!(file, decoded);
    }

    #[test]
    fn bitvec_test() {
        let mut vec = BitVec::new();
        for _ in 0..100 {
            vec.push(One);
            println!("{:#b}", vec.vec[0]);
            vec.push(Zero);
            println!("{:#b}", vec.vec[0]);
            vec.push(One);
            println!("{:#b}", vec.vec[0]);
        }
        println!("{:?}", vec);
    }
}
