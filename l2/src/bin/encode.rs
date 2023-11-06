use std::env;
use std::fs::File;
use std::io::Write;

use l2::*;
use rayon::prelude::*;

fn main() {
    // Take in and vaidate input
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let file = std::fs::read(file_path).unwrap();

    // Calculate entropy
    let symbol_count = file.len();
    let mut occurences: Vec<u32> = vec![0; 256];
    for symbol in &file {
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
    println!("entropy: {:?}", entropy);

    // Encode the file
    let (encoded, tree) = encode(&file);
    println!(
        "compression ratio: {:?}",
        file.len() as f64 / encoded.to_bytes().len() as f64
    );

    // Output encoded file
    let mut output = File::create(output_file_path).unwrap();
    output.write_all(&encoded.to_bytes()).unwrap();

    // Output key to encoded file
    let mut key_file = File::create("key.ron").unwrap();
    key_file
        .write_all(ron::to_string(&tree).unwrap().as_bytes())
        .unwrap();
}
