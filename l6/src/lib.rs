mod pixel;
mod bitvec;

use pixel::Pixel;
use bitvec::*;

fn add_pixel_mod(p0: Pixel, p1:Pixel) -> Pixel {
    Pixel::new(p0[0].wrapping_add(p1[0]), p0[1].wrapping_add(p1[1]), p0[2].wrapping_add(p1[2]))
}
pub fn encode_from_tga(path: &str, bitoffset: u8) -> BitVec {
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
    let mut map: Vec<Pixel> = vec![Pixel::default(); width * height];
    for (x, color_value) in img.chunks(3).enumerate() {
        map[x] = Pixel::new(color_value[0], color_value[1], color_value[2]);
    }
    let mut bitvec = BitVec::new();
    for byte in header {
        bitvec.push_byte(byte);
    }
    bitvec.push_byte(bitoffset);
    let mut prev_pixel = Pixel::default();
    let mut quantized_difference_pixel = Pixel::default();
    for pixel in map {
        for (idx, (sub_pixel, prev_sub_pixel)) in pixel.zip(prev_pixel).enumerate() {
            quantized_difference_pixel[idx] = sub_pixel.wrapping_sub(prev_sub_pixel) >> (8-bitoffset);
            bitvec.push_k_lsb(quantized_difference_pixel[idx], bitoffset);
            quantized_difference_pixel[idx] = quantized_difference_pixel[idx] << (8-bitoffset);
        }
        prev_pixel = add_pixel_mod(prev_pixel, quantized_difference_pixel);
    }
    bitvec
}

pub fn decode_from_bin(path: &str) -> Vec<u8> {
    let file = std::fs::read(path).unwrap();
    let header: Vec<u8> = file[..18].iter().cloned().collect();
    let width = u16::from_le_bytes([file[12], file[13]]) as usize;
    let height = u16::from_le_bytes([file[14], file[15]]) as usize;
    let bitoffset = file[18].clone();
    let mut compressed_img = BitVec::from_bytes(file[19..].iter().cloned().collect());
    let mut prev_pixel = Pixel::default();
    let mut decompressed_img = Vec::new();
    let mut out_file = Vec::new();
    for _ in 0..(width*height) {
        let mut r = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {},
                Bit::One => {
                    r += 1 << n;
                },
            }
        }
        let mut g = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {},
                Bit::One => {
                    g += 1 << n;
                },
            }
        }
        let mut b = 0;
        for n in (0..bitoffset).rev() {
            match compressed_img.next().unwrap() {
                Bit::Zero => {},
                Bit::One => {
                    b += 1 << n;
                },
            }
        }
        let pixel = Pixel::new(r, g, b);
        prev_pixel = add_pixel_mod(prev_pixel, pixel);
        decompressed_img.push(prev_pixel[0]);
        decompressed_img.push(prev_pixel[1]);
        decompressed_img.push(prev_pixel[2]);
    }
    out_file.extend(header);
    out_file.extend(decompressed_img);
    out_file
}
