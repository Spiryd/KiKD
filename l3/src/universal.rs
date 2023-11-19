use crate::bitvec::{Bit::*, *};

pub const EOF: usize  = usize::MAX - 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodingType {
    OMEGA,
    GAMMA,
    DELTA,
    FIB,
}

impl Default for CodingType {
    fn default() -> Self {
        CodingType::OMEGA
    }
}

impl CodingType {
    pub fn from_str(from: &str) -> Option<CodingType> {
        match from {
            "gamma" => Some(CodingType::GAMMA),
            "delta" => Some(CodingType::DELTA),
            "fib" => Some(CodingType::FIB),
            "omega" => Some(CodingType::OMEGA),
            _ => None,
        }
    }

    pub fn encoode(&self, encodee: &Vec<usize>) -> BitVec {
        match self {
            CodingType::OMEGA => omega_encode(encodee),
            CodingType::GAMMA => gamma_encode(encodee),
            CodingType::DELTA => delta_encode(encodee),
            CodingType::FIB => fib_encode(encodee),
        }
    }
    pub fn decode(&self, decodee: BitVec) -> Vec<usize> {
        match self {
            CodingType::OMEGA => omega_decode(decodee),
            CodingType::GAMMA => gamma_decode(decodee),
            CodingType::DELTA => delta_decode(decodee),
            CodingType::FIB => fib_decode(decodee),
        }
    }
}
fn gamma_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    let mut binary_rep = Vec::new();
    
    for code in encodee {
        let mut code = code + 1;
        binary_rep.clear();
        for _ in 0..(usize::BITS - 1 - code.leading_zeros()) {
            bitvec.push(Zero);
        }
        while code != 0 {
            binary_rep.push(code % 2);
            code /= 2;
        }
        binary_rep.iter().rev().for_each(|bit| {
            if *bit == 0 {
                bitvec.push(Zero);
            } else {
                bitvec.push(One);
            }
        });
    }
    bitvec
}
fn gamma_decode(decodee: BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let mut counter = 0;
    let mut current_symbol: usize;
    let mut idx = 0;
    let decodee = decodee.clone().to_vector_of_bits();
    let mut bit = decodee[0];
    loop {
        // Counting zeros
        while bit == Zero {
            counter += 1;
            idx += 1;
            if idx >= decodee.len() - 1 {
                break;
            }
            bit = decodee[idx];
        }
        // outputng number
        current_symbol = 2_usize.pow(counter);
        for _ in 0..counter {
            idx += 1;
            if idx >= decodee.len() - 1 {
                break;
            }
            bit = decodee[idx];
            counter -= 1;
            if bit == One {
                current_symbol += 2_usize.pow(counter);
            }
        }
        if current_symbol - 1 == EOF {
            break;
        }
        result.push(current_symbol - 1);
        // reseting and checking break condition
        counter = 0;
        if idx >= decodee.len() - 1 {
            break;
        }
        idx += 1;
        bit = decodee[idx];
    }
    result
}

fn delta_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    for &code in encodee {
        let code = code + 1;

        let len = 1 + code.ilog2();
        let length_of_len = len.ilog2();

        for _ in 0..length_of_len {
            bitvec.push(Zero);
        }
        for i in (0..=length_of_len).rev() {
            if (len >> i) & 1 == 1 {
                bitvec.push(One);
            } else {
                bitvec.push(Zero);
            }
        }
        for i in (0..(len - 1)).rev() {
            if (code >> i) & 1 == 1 {
                bitvec.push(One);
            } else {
                bitvec.push(Zero);
            }
        }
    }
    bitvec
}
fn delta_decode(decodee: BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let decodee = decodee.clone().to_vector_of_bits();
    let mut idx = 0;
    while idx < decodee.len() {
        let mut bit = decodee[idx];
        let mut num = 1;
        let mut len = 1;
        let mut length_of_len = 0;
        while bit == Zero {
            length_of_len += 1;
            idx += 1;
            bit = decodee[idx];
        }
        for _ in 0..length_of_len {
            len <<= 1;
            idx += 1;
            bit = decodee[idx];
            if bit == One {
                len |= 1;
            }
        }
        for _ in 0..(len - 1) {
            num <<= 1;
            idx += 1;
            bit = decodee[idx];
            if bit == One {
                num |= 1;
            }
        }
        if num - 1 == EOF {
            break;
        } 
        result.push(num - 1);
        idx += 1;
    }
    result
}
fn omega_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    for &code in encodee {
        let mut code = code + 1;
        let mut bit_stack: Vec<Bit> = Vec::new();
        while code > 1 {
            let mut len = 0;
            let mut tmp = code;
            while tmp > 0 {
                len += 1;
                tmp >>= 1;
            }
            for i in 0..len {
                if (code >> i) & 1 == 1 {
                    bit_stack.push(One);
                } else {
                    bit_stack.push(Zero);
                }
            }
            code = len - 1;
        }
        bit_stack.iter().rev().for_each(|bit| bitvec.push(*bit));
        bitvec.push(Zero)
    }
    bitvec
}
fn omega_decode(decodee: BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let decodee = decodee.clone().to_vector_of_bits();
    let mut idx = 0;

    while idx < decodee.len() {
        let mut bit = decodee[idx];
        let mut num = 1;
        while bit == One {
            idx += 1;
            bit = decodee[idx];
            let len = num;
            num = 1;
            for _ in 0..len {
                num <<= 1;
                if bit == One {
                    num |= 1;
                }
                idx += 1;
                bit = decodee[idx];
            }
        }
        if num - 1 == EOF {
            break;
        } 
        result.push(num - 1);
        idx += 1;
    }
    result
}

struct Fibonacci {
    fib: Vec<usize>,
}
impl Fibonacci {
    fn new() -> Fibonacci {
        let fib: Vec<usize> = vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040,
            1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169
        ];
        Fibonacci { fib }
    }
    fn largest_fib_leq(&self, n: usize) -> Option<usize> {
        if self.fib.last().unwrap() >= &n {
            for (i, f) in self.fib.iter().enumerate().rev() {
                if f <= &n {
                    return Some(i);
                }
            }
        }
        None
    }
}

fn fib_encode(encodee: &Vec<usize>) -> BitVec {
    let mut bitvec = BitVec::new();
    let fib = Fibonacci::new();
    for &code in encodee {
        let mut code = code + 1;
        let mut idx = fib.largest_fib_leq(code).unwrap();
        let mut bit_stack: Vec<Bit> = Vec::new();
        while code != 0 {
            bit_stack.push(One);
            code -= fib.fib[idx];
            let new_idx = fib.largest_fib_leq(code).unwrap();
            if new_idx == 0 {
                for _ in 2..idx {
                    bit_stack.push(Zero);
                }
                break;
            }
            for _ in 1..(idx - new_idx) {
                bit_stack.push(Zero);
            }
            idx = new_idx;
        }
        bit_stack.iter().rev().for_each(|&bit| bitvec.push(bit));
        bitvec.push(One);
    }

    bitvec
}
fn fib_decode(decodee: BitVec) -> Vec<usize> {
    let mut result = Vec::new();
    let mut bit_stack = Vec::new();
    let mut one_counter = 0;
    let fib = Fibonacci::new();
    for bit in decodee {
        if bit == One {
            one_counter += 1;
        } else {
            one_counter = 0;
        }
        if one_counter == 2 {
            let mut code = 0;
            for (i, &b) in bit_stack.iter().enumerate() {
                if b == One {
                    code += fib.fib[2 + i];
                }
            }
            result.push(code - 1);
            one_counter = 0;
            bit_stack.clear();
        } else {
            bit_stack.push(bit);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::universal::*;

    #[test]
    fn gamma_test() {
        let encodee: Vec<usize> = (0..10).collect();
        let encoding = gamma_encode(&encodee);
        // encodeing test
        assert_eq!(
            encoding.to_bytes(),
            vec![0b10100110, 0b01000010, 0b10011000, 0b11100010, 0b00000100, 0b10001010]
        );
        // decodeing test
        assert_eq!(encodee, gamma_decode(encoding));
    }

    #[test]
    fn delta_test() {
        let encodee: Vec<usize> = (0..10).collect();
        let encoding = delta_encode(&encodee);
        println!("{:?}", &encoding.to_bytes());
        assert_eq!(encodee, delta_decode(encoding));
    }

    #[test]
    fn omega_test() {
        let encodee: Vec<usize> = (0..10).collect();
        let encoding = omega_encode(&encodee);
        // encodeing test
        assert_eq!(
            encoding.to_bytes(),
            vec![
                0b01001101, 0b01000101, 0b01010110, 0b01011101, 0b11000011, 0b10010111, 0b01000000
            ]
        );
        // decodeing test
        println!("{:?}", encoding.to_bytes());
        assert_eq!(encodee, omega_decode(encoding));
    }

    #[test]
    fn fib_test() {
        let encodee: Vec<usize> = (0..10).collect();
        let encoding = fib_encode(&encodee);
        // encodeing test
        assert_eq!(
            encoding.to_bytes(),
            vec![0b11011001, 0b11011000, 0b11100110, 0b10110000, 0b11100011, 0b01001100]
        );
        // decodeing test
        println!("{:?}", encoding.to_bytes());
        assert_eq!(encodee, fib_decode(encoding));
    }
}
