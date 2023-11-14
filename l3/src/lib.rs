use std::collections::{HashMap, HashSet};

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
    pub fn encode(&self, encodee: &[u8]) -> (Vec<u16>, HashMap<String, u16>) {
        let encodee_len = encodee.len();
        let mut alphabet: HashSet<u8> = HashSet::new();
        //encodee.iter().for_each(|c| {alphabet.insert(*c);});
        for c in 0..255_u8 {
            alphabet.insert(c);
        }
        let mut dictionary: HashMap<String, u16> = HashMap::new();
        alphabet.iter().enumerate().for_each(|(v, k)| {dictionary.insert((*k as char).to_string(), v as u16);});
        //println!("{:?}", &dictionary);
        let mut code: usize = dictionary.len();
        //println!("{:?}", &code);
        let mut p: String = (encodee[0] as char).to_string();
        let mut c: String = String::new();
        //println!("{:?}", &p);
        let mut out: Vec<u16> = Vec::new();
        for i in 0..encodee_len {
            if i != encodee_len - 1 {
                c.push(encodee[i + 1] as char);
            }
            if dictionary.get(&(p.clone() + &c)).is_some() {
                p += &c;
            } else {
                out.push(dictionary[&p]);
                dictionary.insert(p + &c, code as u16);
                code += 1;
                p = c.clone();
            }
            c.clear();
        }
        out.push(dictionary[&p]);
        (out, dictionary)
    }
}

pub struct Decoder {

}

impl Decoder {
    pub fn new() -> Decoder{
        Decoder{}
    }
    pub fn decode(&self, decodee: &[u16]) -> Vec<u8> {
        let encodee_len = decodee.len();
        let mut alphabet: HashSet<u8> = HashSet::new();
        for c in 0..255_u8 {
            alphabet.insert(c);
        }
        let mut dictionary: HashMap<u16, String> = HashMap::new();
        alphabet.iter().enumerate().for_each(|(v, k)| {dictionary.insert(v as u16, (*k as char).to_string());});
        let mut old  = decodee[0];
        let mut n = 0;
        let mut s: String = dictionary[&old].clone();
        let mut c: String = String::new();

        todo!()
    }
}

pub fn entropy(subject: &[u8]) -> f32 {
    let symbol_count = subject.len();
    let mut occurences: Vec<u32> = vec![0; 256];
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
    fn encode_decode_test() {
        let file = std::fs::read("test_cases/test1.bin").unwrap();
        let encoder = Encoder::default();
        println!("{:?}",encoder.encode(&file).0);
        let decoder = Decoder::new();
        assert_eq!(file, decoder.decode(&encoder.encode(&file).0))
    }

    #[test]
    fn conversion_test() {
        let x = 1_u16.to_be_bytes();
        assert_eq!([0_u8, 1_u8], x);
    }
}
