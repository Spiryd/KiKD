use l4::*;

fn main() {
    let test_files = vec![
        "test_cases/example0.tga",
        "test_cases/example1.tga",
        "test_cases/example2.tga",
        "test_cases/example3.tga",
    ];
    for file_path in &test_files {
        println!("case: {}", &file_path[11..]);
        let img = Image::from_tga(file_path);
        //println!("{:?}", &img);
        println!();
    }
}
