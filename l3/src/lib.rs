use std::collections::HashMap;

pub use bitvec::BitVec;
use rayon::prelude::*;

pub mod bitvec;
mod universal;

pub use universal::*;

pub struct Encoder {
    coding_type: CodingType,
}

impl Default for Encoder {
    fn default() -> Self {
        Self {
            coding_type: Default::default(),
        }
    }
}

impl Encoder {
    pub fn new(coding_type: CodingType) -> Encoder {
        Encoder { coding_type }
    }
    /// Implementation of LZW encodeing with the dictionary indices encoded via `coding_type`: [`CodeingType`] universal coding.
    pub fn encode(&self, encodee: &[u8]) -> BitVec {
        let indices = self._encode(encodee);
        self.coding_type.encoode(&indices)
    }

    /// Implementation of LZW encodeing
    fn _encode(&self, encodee: &[u8]) -> Vec<usize> {
        let mut dictionary: HashMap<Vec<u8>, usize> =
            (0..256).map(|i| (vec![i as u8], i)).collect();

        let mut w = Vec::new();
        let mut result = Vec::new();

        for &b in encodee {
            let mut wc = w.clone();
            wc.push(b);

            if dictionary.contains_key(&wc) {
                w = wc;
            } else {
                result.push(dictionary[&w]);

                dictionary.insert(wc, dictionary.len() as usize);
                w.clear();
                w.push(b);
            }
        }

        if !w.is_empty() {
            result.push(dictionary[&w]);
        }

        result
    }
}

pub struct Decoder {
    coding_type: CodingType,
}

impl Default for Decoder {
    fn default() -> Self {
        Self {
            coding_type: Default::default(),
        }
    }
}

impl Decoder {
    pub fn new(coding_type: CodingType) -> Decoder {
        Decoder { coding_type }
    }
    /// Implementation of LZW decodein with the dictionary indices decoded via `coding_type`: [`CodeingType`] universal coding.
    pub fn decode(&self, decodee: BitVec) -> Vec<u8> {
        let indices = self.coding_type.decode(decodee);
        self._decode(&indices)
    }
    /// Implementation of LZW decodeing
    fn _decode(&self, mut decodee: &[usize]) -> Vec<u8> {
        let mut dictionary: HashMap<usize, Vec<u8>> =
            (0..256).map(|i| (i, vec![i as u8])).collect();

        let mut w = dictionary[&decodee[0]].clone();
        decodee = &decodee[1..];
        let mut decompressed = w.clone();

        for &k in decodee {
            let entry = if dictionary.contains_key(&k) {
                dictionary[&k].clone()
            } else if k == dictionary.len() as usize {
                let mut entry = w.clone();
                entry.push(w[0]);
                entry
            } else {
                panic!("Invalid dictionary!");
            };

            decompressed.extend_from_slice(&entry);

            w.push(entry[0]);
            dictionary.insert(dictionary.len() as usize, w);

            w = entry;
        }

        decompressed
    }
}

/// Calculates Entropy of the `subject`
pub fn entropy(subject: &[u8]) -> f32 {
    let symbol_count = subject.len();
    let mut occurences: Vec<usize> = vec![0; 256];
    for symbol in subject {
        occurences[*symbol as usize] += 1;
    }
    let probability: Vec<f32> = occurences
        .par_iter()
        .map(|x| (*x as f32) / (symbol_count as f32))
        .collect();
    let entropy: f32 = probability
        .par_iter()
        .fold(
            || 0.,
            |e: f32, x| if *x == 0.0 { e } else { e - (x * x.log2()) },
        )
        .sum();
    entropy
}

#[cfg(test)]
mod tests {
    use crate::{CodingType::*, *};

    #[test]
    fn simple_encode_decode_test() {
        let file = std::fs::read("test_cases/test0.bin").unwrap();
        let encoder = Encoder::default();
        let decoder = Decoder::default();
        assert_eq!(file, decoder.decode(encoder.encode(&file)))
    }

    #[test]
    fn different_coding_test() {
        let file = std::fs::read("test_cases/test3.bin").unwrap();
        for t in [GAMMA, DELTA, OMEGA, FIB] {
            println!("{:?}", &t);
            let encoder = Encoder::new(t);
            let decoder = Decoder::new(t);
            assert_eq!(file, decoder.decode(encoder.encode(&file)))
        }
    }
}
