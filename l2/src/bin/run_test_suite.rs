use l2::*;
use rayon::prelude::*;

fn main() {
    // Our tests
    let test_files = vec![
        "test_cases/test1.bin",
        "test_cases/test2.bin",
        "test_cases/test3.bin",
        "test_cases/pan-tadeusz-czyli-ostatni-zajazd-na-litwie.txt",
        "test_cases/bee_movie_test_suite/bee_movie_script_x12.txt",
        "test_cases/bee_movie_test_suite/bee_movie_script_x12.doc",
        "test_cases/bee_movie_test_suite/bee_movie_script_x12.pdf",
        "test_cases/bee_movie_test_suite/beemovie.jpg",
        "test_cases/bee_movie_test_suite/beemovie.mp4",
    ];
    for file_path in test_files {
        println!("{file_path}");
        // Ingest file
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
        // Encode Decode Assert and Calculate compression ratio
        let (encoded, key) = encode(&file);
        println!(
            "compression ratio: {:?}",
            file.len() as f64 / encoded.to_bytes().len() as f64
        );
        let decoded = decode(encoded, &key);
        assert_eq!(file, decoded)
    }
}
