use std::collections::HashMap;

use rayon::prelude::*;

pub enum EncodingType {
    OMEGA,
    GAMMA,
    DELTA,
    FIB
}

impl Default for EncodingType {
    fn default() -> Self {
        EncodingType::OMEGA
    }
}

pub struct Encoder {
    encoding_type: EncodingType,
}

impl Default for Encoder {
    fn default() -> Self {
        Self { encoding_type: Default::default() }
    }
}

impl Encoder {
    pub fn new(encoding_type: EncodingType) -> Encoder{
        Encoder{encoding_type}
    }
    pub fn encode(&self, encodee: &[u8]) -> Vec<u16> {
        let mut dictionary: HashMap<Vec<u8>, u16> = (0..256)
            .map(|i| (vec![i as u8], i))
            .collect();

        let mut w = Vec::new();
        let mut result = Vec::new();

        for &b in encodee {
            let mut wc = w.clone();
            wc.push(b);

            if dictionary.contains_key(&wc) {
                w = wc;
            } else {
                result.push(dictionary[&w]);

                dictionary.insert(wc, dictionary.len() as u16);
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

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Decoder{
        Decoder{}
    }
    pub fn decode(&self, mut decodee: &[u16]) -> Vec<u8> {
        let mut dictionary: HashMap::<u16, Vec<u8>> = (0..256)
            .map(|i| (i, vec![i as u8]))
            .collect();

        let mut w = dictionary[&decodee[0]].clone();
        decodee = &decodee[1..];
        let mut decompressed = w.clone();

        for &k in decodee {
            let entry = if dictionary.contains_key(&k) {
                dictionary[&k].clone()
            } else if k == dictionary.len() as u16 {
                let mut entry = w.clone();
                entry.push(w[0]);
                entry
            } else {
                panic!("Invalid dictionary!");
            };

            decompressed.extend_from_slice(&entry);

            w.push(entry[0]);
            dictionary.insert(dictionary.len() as u16, w);

            w = entry;
        }

        decompressed
    }
}

pub fn entropy(subject: &[u8]) -> f32 {
    let symbol_count = subject.len();
    let mut occurences: Vec<u16> = vec![0; 256];
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
    use crate::*;

    #[test]
    fn simple_encode_decode_test() {
        let file = std::fs::read("test_cases/test1.txt").unwrap();
        let encoder = Encoder::default();
        let decoder = Decoder::new();
        assert_eq!(file, decoder.decode(&encoder.encode(&file)))
    }

    #[test]
    fn conversion_test() {
        let x = 1_u16.to_be_bytes();
        assert_eq!([0_u8, 1_u8], x);
    }
}
