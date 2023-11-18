use std::env;
use std::fs::File;
use std::io::Write;

use l3::{*, CodingType::*};

fn main() {
        // Take in and vaidate input
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            panic!("Wrong argument count")
        }
        let encoding_type = match args.get(3) {
            Some(enc_str) => {
                match enc_str.as_str() {
                    "gamma" => GAMMA,
                    "delta" => DELTA,
                    "fib" => FIB,
                    _ => OMEGA,
                }
            },
            None => OMEGA,
        };
        let file_path = args.get(1).unwrap();
        let output_file_path = args.get(2).unwrap();
        let file = std::fs::read(file_path).unwrap();

        println!("entropy: {:?}", entropy(&file));

        let encoder = Encoder::new(encoding_type);
        let encoded = encoder.encode( &file);

        let mut output = File::create(output_file_path).unwrap();
        output.write_all(&encoded.to_bytes()).unwrap();
}
