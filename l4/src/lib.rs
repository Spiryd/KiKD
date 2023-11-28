use rayon::prelude::*;

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
        let mut map: Vec<Vec<Pixel>> = vec![vec![[0, 0, 0]; width ]; height];
        let mut y = 0;
        for (x, color_value) in img.chunks(3).enumerate() {
            map[y][x % width] = [color_value[0], color_value[1], color_value[2]];
            if x % width == width - 1 {
                y += 1;
            }
        }
        Image { width, height, map }
    }

    pub fn entropy(&self) {
        let mut reds = Vec::new();
        let mut greens = Vec::new();
        let mut blues = Vec::new();
        let mut all = Vec::new();
        for row in &self.map {
            for pixel in row {
                reds.push(pixel[0]);
                greens.push(pixel[1]);
                blues.push(pixel[2]);

                all.push(pixel[0]);
                all.push(pixel[1]);
                all.push(pixel[2]);
            }
        }
        println!("Image entropy: {}", entropy(&all));
        println!("red entropy: {}", entropy(&reds));
        println!("greens entropy: {}", entropy(&greens));
        println!("blues entropy: {}", entropy(&blues));
    }

    pub fn encode(&self, predictor: Predictor) -> (f32, f32, f32, f32) {
        let prediction = match predictor {
            Predictor::One => self.predicton_1(),
            Predictor::Two => self.predicton_2(),
            Predictor::Three => self.predicton_3(),
            Predictor::Four => self.predicton_4(),
            Predictor::Five => self.predicton_5(),
            Predictor::Six => self.predicton_6(),
            Predictor::Seven => self.predicton_7(),
            Predictor::New => self.predicton_new(),
        };
        let diff = self.diff(prediction);
        _entropy(diff)
    }

    pub fn diff(&self, prediction: Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
        let mut diff: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let red_diff = self.map[y][x][0].abs_diff(prediction[y][x][0]);
                let green_diff = self.map[y][x][1].abs_diff(prediction[y][x][1]);
                let blue_diff = self.map[y][x][2].abs_diff(prediction[y][x][2]);
                diff[y][x] =  [red_diff, green_diff, blue_diff];
            }
        }
        diff
    }

    fn predicton_1(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 0..self.height {
            for x in 1..self.width {
                prediction[y][x] = self.map[y][x - 1];
            }
        }
        prediction
    }

    fn predicton_2(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 0..self.width {
                prediction[y][x] = self.map[y - 1][x];
            }
        }
        prediction
    }

    fn predicton_3(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
            vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 1..self.width {
                prediction[y][x] = self.map[y - 1][x - 1];
            }
        }
        prediction
    }

    fn predicton_4(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.map[0][x -1];
        }
        for y in 1..self.height {
            prediction[y][0] = self.map[y - 1][0];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.map[y - 1][x];
                let west = self.map[y][x -1];
                let north_west = self.map[y - 1][x];
                prediction[y][x] = [north[0] + west[0] - north_west[0], north[1] + west[1] - north_west[1], north[2] + west[2] - north_west[2]];
            }
        }
        prediction
    }

    fn predicton_5(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            let west = self.map[0][x - 1];
            prediction[0][x] = [west[0]/2, west[1]/2, west[2]/2];
        }
        for y in 1..self.height {
            prediction[y][0] = self.map[y - 1][0];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.map[y - 1][x];
                let west = self.map[y][x -1];
                let north_west = self.map[y - 1][x];
                prediction[y][x] = [north[0] + (west[0] - north_west[0])/2, north[1] + (west[1] - north_west[1])/2, north[2] + (west[2] - north_west[2])/2];
            }
        }
        prediction
    }

    fn predicton_6(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.map[0][x-1];
        }
        for y in 1..self.height {
            let north = self.map[y-1][0];
            prediction[y][0] = [north[0]/2, north[1]/2, north[2]/2];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.map[y - 1][x];
                let west = self.map[y][x -1];
                let north_west = self.map[y - 1][x];
                prediction[y][x] = [west[0] + (north[0] - north_west[0])/2, west[1] + (north[1] - north_west[1])/2, west[2] + (north[2] - north_west[2])/2];
            }
        }
        prediction
    }

    fn predicton_7(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for x in 1..self.width {
            prediction[0][x] = self.map[0][x - 1];
        }
        for y in 1..self.height {
            let north = self.map[y][0];
            prediction[y][0] = [north[0]/2, north[1]/2, north[2]/2];
        }
        for y in 1..self.height {
            for x in 1..self.width {
                let north = self.map[y - 1][x];
                let west = self.map[y][x -1];
                let north_west = self.map[y - 1][x];
                prediction[y][x] = [west[0] + (north[0] - north_west[0])/2, west[1] + (north[1] - north_west[1])/2, west[2] + (north[2] - north_west[2])/2];
            }
        }
        prediction
    }

    fn predicton_new(&self) -> Vec<Vec<Pixel>> {
        let mut prediction: Vec<Vec<Pixel>> =
        vec![vec![[0, 0, 0]; self.width]; self.height];
        for y in 1..self.height {
            for x in 1..self.width {
                let north = if y == 0 {
                    [0, 0, 0]
                } else {
                    self.map[y - 1][x]
                };
                let west = if x == 0 {
                    [0, 0, 0]
                } else {
                    self.map[y][x - 1]
                };
                let north_west = if x == 0 || y ==0 {
                    [0, 0, 0]
                } else {
                    self.map[y - 1][x - 1]
                };
                let mut pixel = [0, 0, 0];
                for c in 0..3 {
                    if north_west[c] >= west[c].max(north[c]) {
                        pixel[c] = west[c].max(north[c]);
                    } else if north_west[c] <= west[c].min(north[c]) {
                        pixel[c] = west[c].min(north[c]);
                    } else {
                        pixel[c] = north[c] + west[c] - north_west[c] ;
                    }
                }
                prediction[y][x] = pixel;
            }
        }
        prediction
    }
}

fn _entropy(subject: Vec<Vec<Pixel>>) -> (f32, f32, f32, f32) {
    let mut reds = Vec::new();
    let mut greens = Vec::new();
    let mut blues = Vec::new();
    let mut all = Vec::new();
    for row in subject {
        for pixel in row {
            reds.push(pixel[0]);
            greens.push(pixel[1]);
            blues.push(pixel[2]);

            all.push(pixel[0]);
            all.push(pixel[1]);
            all.push(pixel[2]);
        }
    }
    (entropy(&all), entropy(&reds), entropy(&greens), entropy(&blues))
}

#[derive(Debug, Clone, Copy)]
pub enum Predictor {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    New
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
