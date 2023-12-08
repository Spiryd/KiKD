use std::{fmt::Debug, ops::Index};

#[derive(Clone, Copy, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({},{},{})", self.r, self.g, self.b))
    }
}

impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("There are only 3 color values my dude"),
        }
    }
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{r, g, b}
    }
    pub fn dist(&self, other: &Pixel) -> usize{
        self[0].abs_diff(other[0]) as usize + self[1].abs_diff(other[1]) as usize  + self[2].abs_diff(other[2]) as usize
    }
    pub fn perturbation(&self, epsilon: u8) -> (Pixel, Pixel) {
        let r = (self[0].saturating_add(epsilon), self[0].saturating_sub(epsilon));
        let g = (self[1].saturating_add(epsilon), self[1].saturating_sub(epsilon));
        let b = (self[2].saturating_add(epsilon), self[2].saturating_sub(epsilon));
        (Pixel::new(r.0, g.0, b.0), Pixel::new(r.1, g.1, b.1))
    }
    pub fn to_bytes_rgb(&self) -> Vec<u8> {
        vec![self[0], self[1], self[2]]
    }
    pub fn to_bytes_brg(&self) -> Vec<u8> {
        vec![self[2], self[1], self[0]]
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}
