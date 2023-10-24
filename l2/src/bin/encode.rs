use std::env;

use l2::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args.get(1);
    match file_path {
        Some(file_path) => {
            let file = std::fs::read(file_path).unwrap();
            encode_arithmetic(&file);
        },
        None => panic!("Flie path not supplied"),
    }
}