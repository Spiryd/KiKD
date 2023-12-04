use rand::prelude::*;
use rand_pcg::Pcg64;

type Pixel = [u8; 3];

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn from_tga(path: &str) -> Image {
        let file = std::fs::read(path).unwrap();
        let width = u16::from_le_bytes([file[12], file[13]]) as usize;
        let height = u16::from_le_bytes([file[14], file[15]]) as usize;
        println!("width: {}", &width);
        println!("height: {}", &height);
        let depth = file[16];
        println!("depth: {}", &depth);
        let img = &file[18..(3 * width * height + 18)];
        println!("image size: {}B", img.len());
        let mut map: Vec<Vec<Pixel>> = vec![vec![[0, 0, 0]; width]; height];
        let mut y = 0;
        for (x, color_value) in img.chunks(3).enumerate() {
            map[y][x % width] = [color_value[0], color_value[1], color_value[2]];
            if x % width == width - 1 {
                y += 1;
            }
        }
        Image { width, height, map }
    }
    pub fn quantization(&self, cluster_count: usize) -> Vec<Vec<Pixel>> {
        let mut points: Vec<Pixel> = Vec::new();
        for row in &self.map {
            for &pixel in row {
                points.push(pixel);
            }
        }
        let mut rng = Pcg64::from_entropy();
        let clusters: Vec<Pixel> = points.choose_multiple(&mut rng, cluster_count).cloned().collect();

        todo!()
    }
}
