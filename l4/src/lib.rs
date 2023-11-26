use rayon::prelude::*;

type Pixel = (u8, u8, u8);

const BACKGROUND: Pixel = (0, 0, 0);

#[derive(Debug)]
pub struct Image {
    width: u16,
    height: u16,
    map: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn from_tga(path: &str) -> Image {
        let file = std::fs::read(path).unwrap();
        let width = u16::from_le_bytes([file[12], file[13]]);
        let height = u16::from_le_bytes([file[14], file[15]]);
        println!("width: {}", &width);
        println!("height: {}", &height);
        let depth = file[16];
        println!("depth: {}", &depth);
        let img = &file[18..(3 * width as usize * height as usize + 18)];
        println!("image size: {}B", img.len());
        let mut map: Vec<Vec<Pixel>> = vec![vec![(0, 0, 0); width as usize]; height as usize];
        let mut y = 0;
        for (x, color_value) in img.chunks(3).enumerate() {
            map[y][x % width as usize] = (color_value[0], color_value[1], color_value[2]);
            if x % width as usize ==  width as usize - 1 {
                y += 1;
            }
        }
        Image { width, height, map }
    }
    fn predicton_1(&self) {
        todo!()
    }
    
    fn predicton_2(&self) {
        todo!()
    }
    
    fn predicton_3(&self) {
        todo!()
    }
    fn predicton_4(&self) {
        todo!()
    }
    fn predicton_5(&self) {
        todo!()
    }
    fn predicton_6(&self) {
        todo!()
    }
    fn predicton_7(&self) {
        todo!()
    }
    fn predicton_new(&self) {
        todo!()
    }
}

/// Calculates Entropy of the `subject`
pub fn entropy(subject: &[u8]) -> f32 {
    let symbol_count = subject.len();
    let mut occurences: Vec<usize> = vec![0; 256];
    for symbol in subject {
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
    entropy
}

