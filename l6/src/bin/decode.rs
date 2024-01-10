use std::env;
use std::fs::File;
use std::io::Write;

use l6::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let img = decode_from_bin(file_path);
    let mut output_file = File::create(output_file_path).expect("Unable to create file");
    output_file.write_all(&img).expect("Unable to write data");
}