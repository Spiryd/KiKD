use std::env;
use std::fs::File;
use std::io::Write;

use l5::Image;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Wrong argument count")
    }
    let file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    let color_count: u8 = args.get(3).unwrap().parse().unwrap();
    let img = Image::from_tga(file_path);
}
