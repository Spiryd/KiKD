use std::env;

use l1::entropy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::read(file_path).unwrap();

    entropy(file);
}
