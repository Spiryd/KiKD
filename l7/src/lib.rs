use std::ops::{BitXor, BitXorAssign};

use Bit::*;

pub fn hamming_encoding(data: Vec<Bit>) -> Result<Vec<Bit>, String> {
    if data.len() != 4 {
        return Err("Data too long".to_string());
    }
    let p1 = data[0] ^ data[1] ^ data[3];
    let p2 = data[0] ^ data[2] ^ data[3];
    let p3 = data[1] ^ data[2] ^ data[3];
    let p4 = data[0] ^ data[1] ^ data[2] ^ data[3] ^ p1 ^ p2 ^ p3;
    Ok(vec![p1, p2, data[0], p3, data[1], data[2], data[3], p4])
}

pub fn hamming_decoding(mut data: Vec<Bit>) -> Result<Vec<Bit>, Option<Vec<Bit>>> {
    if data.len() != 8 {
        return Err(None);
    }
    let p1 = data[0];
    let p2 = data[1];
    let d1 = data[2];
    let p3 = data[3];
    let d2 = data[4];
    let d3 = data[5];
    let d4 = data[6];
    let p4 = data[7];

    let p1_calc = d1 ^ d2 ^ d4;
    let p2_calc = d1 ^ d3 ^ d4;
    let p3_calc = d2 ^ d3 ^ d4;
    let p4_calc = d1 ^ d2 ^ d3 ^ d4 ^ p1 ^ p2 ^ p3;

    let mut error_pos = 0;
    if p1 != p1_calc {
        error_pos += 1;
    }
    if p2 != p2_calc {
        error_pos += 2;
    }
    if p3 != p3_calc {
        error_pos += 4;
    }

    if p4 != p4_calc {
        if error_pos == 0 {
            data[7] ^= One; // Correct the error in the parity bit
        } else {
            data[error_pos - 1] ^= One; // Correct the error
        }
    } else if error_pos != 0 {
        return Err(Some(vec![d1, d2, d3, d4])); // Double bit error detected
    }

    Ok(vec![data[2], data[4], data[5], data[6]]) // Return the data bits
}

/// Enum symbolising bits
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn to_u8(&self) -> u8 {
        match self {
            Zero => 0,
            One => 1,
        }
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Zero, Zero) => Zero,
            (One, One) => Zero,
            (Zero, One) => One,
            (One, Zero) => One,
        }
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

/// BitVector Implementation
/// # Fields
/// - `vec`: vector of bytes
/// - `inside_idx`: index of the next bit to be pushed
/// - `outside_idx`: index of the current byte
/// - `inside_itr_couner`: index of the next bit to be outputed
/// - `outside_itr_couner`: index of the current byte for iterateing
#[derive(Debug, Clone)]
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
        if self.outside_idx == self.outside_itr_couner && self.inside_idx == self.inside_itr_couner
        {
            self.inside_itr_couner = 0;
            self.outside_itr_couner = 0;
            return None;
        }
        let output: Bit =
            if (self.vec[self.outside_itr_couner] & (0x80 >> self.inside_itr_couner)) > 0 {
                One
            } else {
                Zero
            };
        self.inside_itr_couner += 1;
        Some(output)
    }
}

impl BitVec {
    /// Creates a new [BitVec] with initialised first byte
    pub fn new() -> BitVec {
        BitVec {
            vec: vec![0],
            inside_idx: 0,
            outside_idx: 0,
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }
    /// Creates a [BitVec] from a vector of bytes
    pub fn from_bytes(bytes: Vec<u8>) -> BitVec {
        BitVec {
            vec: bytes.clone(),
            inside_idx: 0,
            outside_idx: bytes.len(),
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }
    /// Outputs a vector of bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.vec.clone()
    }
    /// Outputs a vector of [Bit]s
    pub fn to_vector_of_bits(&mut self) -> Vec<Bit> {
        let mut vector_of_bits = Vec::new();
        for bit in self {
            vector_of_bits.push(bit);
        }
        vector_of_bits
    }
    /// Pushes the supplied `bit`:[Bit] to our vector
    pub fn push(&mut self, bit: Bit) {
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
    pub fn push_byte(&mut self, byte: u8) {
        for n in (0..8).rev() {
            if (byte & (1 << n)) > 0 {
                self.push(One);
            } else {
                self.push(Zero)
            }
        }
    }
    pub fn push_k_lsb(&mut self, byte: u8, k: u8) {
        for n in (0..k).rev() {
            if (byte & (1 << n)) > 0 {
                self.push(One);
            } else {
                self.push(Zero)
            }
        }
    }
}

#[cfg(test)]
mod hamming_tests {
    use super::*;

    #[test]
    fn hamming_encoding_test() {
        let data = vec![One, Zero, One, Zero];
        let res = hamming_encoding(data);
        assert_eq!(res, Ok(vec![One, Zero, One, One, Zero, One, Zero, Zero]));
    }

    #[test]
    fn hamming_decoding_noerror_test() {
        let decodee = vec![One, Zero, One, One, Zero, One, Zero, Zero];
        let res = hamming_decoding(decodee);
        assert!(res.is_ok());
        assert_eq!(res, Ok(vec![One, Zero, One, Zero]));
    }

    #[test]
    fn hamming_decoding_correcable_test() {
        let decodee = vec![One, Zero, One, One, Zero, One, One, Zero];
        let res = hamming_decoding(decodee);
        assert!(res.is_ok());
        assert_eq!(res, Ok(vec![One, Zero, One, Zero]));
    }

    #[test]
    fn hamming_decoding_uncorrecable_test() {
        let decodee = vec![One, Zero, One, One, Zero, Zero, One, Zero];
        let res = hamming_decoding(decodee);
        assert!(res.is_err());
        assert_eq!(res, Err(Some(vec![One, Zero, Zero, One])));
    }

    #[test]
    fn hamming_decoding_no_errors_wrong_parity_bit_test() {
        let decodee = vec![One, Zero, One, One, Zero, One, Zero, One]; // No errors, but wrong parity bit
        let res = hamming_decoding(decodee);
        assert!(res.is_ok());
        assert_eq!(res, Ok(vec![One, Zero, One, Zero]));
    }

    #[test]
    fn hamming_decoding_error_in_parity_bit_test() {
        let decodee = vec![One, Zero, One, One, Zero, One, Zero, One]; // Error in parity bit
        let res = hamming_decoding(decodee);
        assert!(res.is_ok());
        assert_eq!(res, Ok(vec![One, Zero, One, Zero]));
    }

    #[test]
    fn hamming_decoding_double_error_cancelling_test() {
        let decodee = vec![One, One, One, One, Zero, One, Zero, Zero]; // Double error cancelling itself out
        let res = hamming_decoding(decodee);
        assert!(res.is_ok());
        assert_eq!(res, Ok(vec![One, Zero, One, Zero]));
    }

    #[test]
    fn hamming_encoding_empty_test() {
        let data = vec![];
        let res = hamming_encoding(data);
        assert!(res.is_err());
    }

    #[test]
    fn hamming_decoding_empty_test() {
        let decodee = vec![];
        let res = hamming_decoding(decodee);
        assert!(res.is_err());
    }

    #[test]
    fn hamming_encoding_large_input_test() {
        let data = vec![One; 10000];
        let res = hamming_encoding(data);
        assert!(res.is_err());
    }

    #[test]
    fn hamming_decoding_large_input_test() {
        let decodee = vec![One; 10000];
        let res = hamming_decoding(decodee);
        assert!(res.is_err());
    }

    #[test]
    fn hamming_encoding_random_input_test() {
        let data: Vec<Bit> = (0..10000)
            .map(|_| if rand::random() { One } else { Zero })
            .collect();
        let res = hamming_encoding(data);
        assert!(res.is_err());
    }

    #[test]
    fn hamming_decoding_random_input_test() {
        let decodee: Vec<Bit> = (0..10000)
            .map(|_| if rand::random() { One } else { Zero })
            .collect();
        let res = hamming_decoding(decodee);
        assert!(res.is_err());
    }
}
