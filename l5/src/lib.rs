use std::{fmt::Debug, ops::Index};

use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pixel").field("r", &self.r).field("g", &self.g).field("b", &self.b).finish()
    }
}

impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Only 3 color values, get serious my dude"),
        }
    }
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{r, g, b}
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

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
        let img: Vec<u8> = file[18..(3 * width * height + 18)].into_iter().rev().map(|x| *x).collect();
        println!("image size: {}B", img.len());
        let mut map: Vec<Vec<Pixel>> = vec![vec![Pixel::default(); width]; height];
        let mut y = 0;
        for (x, color_value) in img.chunks(3).enumerate() {
            map[y][x % width] = Pixel::new(color_value[0], color_value[1], color_value[2]);
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
