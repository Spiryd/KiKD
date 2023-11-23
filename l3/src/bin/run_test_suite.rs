use l3::{CodingType::*, *};
use std::time::Instant;

fn main() {
    // Our tests
    let test_files = vec![
        "test_cases/test1.bin",
        "test_cases/test2.bin",
        "test_cases/test3.bin",
        "test_cases/pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt",
        //"test_cases/bee_movie_test_suite/bee_movie_script_x12.txt",
        //"test_cases/bee_movie_test_suite/bee_movie_script_x12.doc",
        //"test_cases/bee_movie_test_suite/bee_movie_script_x12.pdf",
        //"test_cases/bee_movie_test_suite/beemovie.jpg",
        //"test_cases/bee_movie_test_suite/beemovie.mp4",
    ];
    for coding in [GAMMA, DELTA, OMEGA, FIB] {
        for file_path in &test_files {
            let file = std::fs::read(file_path).unwrap();
            println!("file: {:?}", &file_path);
            println!("encoding: {:?}", &coding);
            println!("encodee size: {:?} B", file.len());
            println!("encodee entropy: {:?}", entropy(&file));
        
            let encoder = Encoder::new(coding);
            let now = Instant::now();
            let encoded = encoder.encode(&file, false);
            let elapsed = now.elapsed();
            let encoded_bytes = encoded.to_bytes();
            println!("encoding size: {:?} B", encoded_bytes.len());
            println!("encoding entropy: {:?}", entropy(&encoded_bytes));
            println!(
                "compression ratio: {:?}",
                file.len() as f64 / encoded_bytes.len() as f64
            );
            println!("Elapsed: {:.2?}", elapsed);
            println!("\n");
        }
    }
}