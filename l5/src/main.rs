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
    let color_count: usize = args.get(3).unwrap().parse().unwrap();
    let img = Image::from_tga(file_path);
    let codebook = img.quantization(color_count);
    let quantized_img = img.codebook_to_tga(&codebook);
    let mut output = File::create(output_file_path).unwrap();
    output.write_all(&quantized_img).unwrap();
}
