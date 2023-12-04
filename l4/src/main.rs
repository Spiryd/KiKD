use l4::{*, Predictor::*};

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
        println!("original image: ");
        img.entropy();
        let mut all: Vec<(String, f32)> = Vec::new();
        let mut red: Vec<(String, f32)> = Vec::new();
        let mut green: Vec<(String, f32)> = Vec::new();
        let mut blue: Vec<(String, f32)> = Vec::new();
        for predictor in [One, Two, Three, Four, Five, Six, Seven, New] {
            println!("encoded image with predictor {:?}: ", predictor);
            let entropies = img.encode(predictor);
            println!("{:?}", &entropies);
            all.push((format!("{:?}", predictor), entropies.0));
            red.push((format!("{:?}", predictor), entropies.1));
            green.push((format!("{:?}", predictor), entropies.2));
            blue.push((format!("{:?}", predictor), entropies.3));
        }
        println!("Best all: {:?}", all.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap());
        println!("Best red: {:?}", red.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap());
        println!("Best green: {:?}", green.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap());
        println!("Best blue: {:?}", blue.iter().max_by(|x, y| x.1.total_cmp(&y.1)).unwrap());
        println!();
    }
}
