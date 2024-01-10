use std::env;
use std::fs::File;
use std::io::Write;

use l6::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let bitoffset: u8 = args.get(3).unwrap().parse().unwrap();
    let img = encode_from_tga(file_path, bitoffset);
    let mut output_file = File::create(output_file_path).expect("Unable to create file");
    output_file.write_all(&img.to_bytes()).expect("Unable to write data");
}