use std::env;
use std::fs::File;
use std::io::Write;

use l3::*;

fn main() {
    // Take in and vaidate input
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let file = std::fs::read(file_path).unwrap();
    let decoder = Decoder::new();
}