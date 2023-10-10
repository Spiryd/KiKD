use std::env;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::read(file_path).unwrap();
    //println!("{:?}", file.len());
    let mut occurences: Vec<u32> =  vec![0; u8::MAX as usize];
    for symbol in file {
        occurences[symbol as usize] += 1;
    }
    dbg!(&occurences);
}
