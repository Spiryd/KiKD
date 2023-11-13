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
    pub fn encode(&self) {
        todo!()
    }
}

pub struct Decoder {

}

impl Decoder {
    pub fn new() -> Decoder{
        Decoder{}
    }
    pub fn decode(&self) {
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
