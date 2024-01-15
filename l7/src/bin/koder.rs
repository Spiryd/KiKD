use std::env;
use std::fs::File;
use std::io::Write;

use l7::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }
    let input_file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let input_file = BitVec::from_bytes(std::fs::read(input_file_path).unwrap());
}
