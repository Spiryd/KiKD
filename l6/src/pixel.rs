use std::{fmt::Debug, ops::{Index, IndexMut}};

#[derive(Clone, Copy, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    iter: u8,
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

impl IndexMut<usize> for Pixel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("There are only 3 color values my dude"),
        }
    }
}

impl Iterator for Pixel {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter {
            0 => {
                self.iter += 1;
                Some(self.r)
            },
            1 => {
                self.iter += 1;
                Some(self.g)
            },
            2 => {
                self.iter += 1;
                Some(self.b)
            },
            _ => {
                self.iter = 0;
                None
            },
        }
    }
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{r, g, b, iter: 0}
    }
    pub fn dist(&self, other: &Pixel) -> usize{
        self[0].abs_diff(other[0]) as usize + self[1].abs_diff(other[1]) as usize  + self[2].abs_diff(other[2]) as usize
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, iter: 0}
    }
}
