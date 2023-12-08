

mod pixel;

use pixel::Pixel;

const EPSILON: u8 = 3;

#[derive(Debug)]
pub struct Image {
    header: Vec<u8>,
    footer: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<Pixel>>,
}

impl Image {
    pub fn from_tga(path: &str) -> Image {
        let file = std::fs::read(path).unwrap();
        let header: Vec<u8> = file[..18].iter().cloned().collect();
        let width = u16::from_le_bytes([file[12], file[13]]) as usize;
        let height = u16::from_le_bytes([file[14], file[15]]) as usize;
        let footer: Vec<u8> = file[(3 * width * height + 18)..].iter().cloned().collect();
        println!("width: {}", &width);
        println!("height: {}", &height);
        let depth: u8 = file[16];
        println!("depth: {}", &depth);
        let img = &file[18..(3 * width * height + 18)];
        println!("image size: {}B", img.len());
        let mut map: Vec<Vec<Pixel>> = vec![vec![Pixel::default(); width]; height];
        let mut y = 0;
        for (x, color_value) in img.chunks(3).enumerate() {
            map[y][x % width] = Pixel::new(color_value[0], color_value[1], color_value[2]);
            if x % width == width - 1 {
                y += 1;
            }
        }
        Image { header, footer, width, height, map }
    }
    pub fn quantization(&self, color_count: usize) -> Vec<Pixel> {
        let cluster_count = 2_usize.pow(color_count as u32);
        let training_vectors: Vec<Pixel> = self.map.iter().flatten().cloned().collect();
        let mut codebook: Vec<Pixel> = Vec::new();
        let c_0 = avg_vec(&training_vectors);
        //println!("{:?}", &c_0);
        codebook.push(c_0);
        while codebook.len() < cluster_count {
            codebook = lgb(&training_vectors, &codebook);
            println!("{:?}", &codebook);
        }
        codebook
    }
    pub fn codebook_to_tga(&self, codebook: &[Pixel]) -> Vec<u8> {
        let vectors: Vec<Pixel> = self.map.iter().flatten().cloned().collect();
        let mut output = Vec::new();
        output.append(&mut self.header.clone());
        for vector in vectors {
            let coded = codebook.iter().min_by_key(|c| c.dist(&vector)).unwrap().to_bytes_rgb();
            output.append(&mut coded.clone());
        }
        output.append(&mut self.footer.clone());
        output
    }
}

fn avg_vec(vecs: &[Pixel]) -> Pixel {
    let size = vecs.len() as f64;
    let mut avg_vec = (0_f64, 0_f64, 0_f64);
    for vector in vecs {
        avg_vec.0 += vector[0] as f64 / size;
        avg_vec.1 += vector[1] as f64 / size;
        avg_vec.2 += vector[2] as f64 / size;
    }
    Pixel::new(avg_vec.0 as u8, avg_vec.1 as u8, avg_vec.2 as u8)
}

fn lgb(training_vectors: &[Pixel], codebook: &Vec<Pixel>) -> Vec<Pixel> {
    let mut prev_distortion = 0;
    let mut new_codebook = Vec::new();
    for c in codebook {
        let perturbation = c.perturbation(1);
        new_codebook.push(perturbation.0);
        new_codebook.push(perturbation.1);
    }
    loop {
        let mut clusters: Vec<Vec<Pixel>> = vec![Vec::new(); new_codebook.len()];
        for vector in training_vectors {
            let assignment = new_codebook.iter().map(|centroid| vector.dist(centroid)).enumerate().min_by_key(|(_idx, dist)| *dist).unwrap().0;
            clusters[assignment].push(*vector);
        }
        let mut current_distortion = 0;
        for (idx, cluster) in clusters.iter().enumerate() {
            current_distortion += cluster.iter().map(|vector| new_codebook[idx].dist(vector)).sum::<usize>();
        }
        if ((current_distortion as f64 - prev_distortion as f64)/current_distortion as f64).abs() < 0.001 {
           break; 
        }
        prev_distortion = current_distortion;
        new_codebook = clusters.iter().map(|cluster| avg_vec(cluster)).collect();
    }
    new_codebook
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn imige_ingest_test() {
        for path in [
            "test_cases/example0.tga",
            "test_cases/example1.tga",
            "test_cases/example2.tga",
            "test_cases/example3.tga",
        ] {
            Image::from_tga(path);
        }
    }
    #[test]
    fn color_test() {
        let img = Image::from_tga("test_cases/example1.tga");
        assert_eq!(img.map[0][0], Pixel::new(0, 0, 0));
    }
    #[test]
    fn quantization_test() {
        for path in [
            "test_cases/example0.tga",
            "test_cases/example1.tga",
            "test_cases/example2.tga",
            "test_cases/example3.tga",
        ] {
            let img = Image::from_tga(path);
            println!("{:?}",  img.quantization(4));
        }

    }
}
