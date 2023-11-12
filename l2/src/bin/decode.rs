use std::env;
use std::fs::File;
use std::io::Write;

use l2::*;

fn main() {
    // Take in and vaidate input
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(3).unwrap();
    let key_path = args.get(2).unwrap();
    let input_file = std::fs::read(file_path).unwrap();
    // Make bitvec form file
    let bitvec = BitVec::from_bytes(input_file);
    // Deserialise key
    let key: HuffmanTree = ron::from_str(&std::fs::read_to_string(key_path).unwrap()).unwrap();
    // Decode file and Output it
    let mut output_file = File::create(output_file_path).unwrap();
    output_file.write_all(&decode(bitvec, &key)).unwrap();
}
