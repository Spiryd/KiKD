use std::collections::BinaryHeap;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use Bit::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bit {
    Zero,
    One,
}

impl Default for Bit {
    fn default() -> Self {
        Zero
    }
}

#[derive(Debug)]
pub struct BitVec {
    pub vec: Vec<u8>,
    inside_idx: usize,
    outside_idx: usize,
    inside_itr_couner: usize,
    outside_itr_couner: usize,
}

impl Iterator for BitVec {
    type Item = Bit;
    fn next(&mut self) -> Option<Self::Item> {
        if self.inside_itr_couner == 8 {
            self.inside_itr_couner = 0;
            self.outside_itr_couner += 1;
        }
        if self.outside_idx < self.outside_itr_couner {
            self.inside_itr_couner = 0;
            self.outside_itr_couner = 0;
            return None;
        }
        let output: Bit = if (self.vec[self.outside_itr_couner]
            & (0x80 >> self.inside_itr_couner))
            > 0
        {
            One
        } else {
            Zero
        };
        self.inside_itr_couner += 1;
        Some(output)
    }
}

impl BitVec {
    pub fn new() -> BitVec {
        BitVec {
            vec: vec![0],
            inside_idx: 0,
            outside_idx: 0,
            inside_itr_couner: 0,
            outside_itr_couner: 0,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> BitVec {
        let mut x = [0_u8; 8];
        for (i, b) in bytes.iter().take(8).enumerate() {
            x[i] = *b;
        }
        let inside_idx = u64::from_be_bytes(x) as usize;
        BitVec { vec: bytes.iter().skip(8).map(|x| *x).collect(), inside_idx, outside_idx: bytes.len() - 9, inside_itr_couner: 0, outside_itr_couner: 0 }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for b in self.inside_idx.to_be_bytes() {
            output.push(b)
        }
        output.append(&mut self.vec.clone());
        output
    }

    fn push(&mut self, bit: Bit) {
        if self.inside_idx == 8 {
            self.inside_idx = 0;
            self.outside_idx += 1;
            self.vec.push(0);
        }
        match bit {
            Bit::Zero => (),
            Bit::One => self.vec[self.outside_idx] ^= 0x80 >> self.inside_idx,
        }
        self.inside_idx += 1;
    }
}


#[derive(Deserialize, Serialize)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum HuffmanTree {
    Node { freq: usize, left: Box<HuffmanTree>, right: Box<HuffmanTree> },
    Leaf { value: u8, freq: usize },
}

impl HuffmanTree {
    fn frequency(&self) -> usize {
        match self {
            HuffmanTree::Node { freq, .. } => *freq,
            HuffmanTree::Leaf { freq, .. } => *freq,
        }
    }
}

fn build_huffman_tree(frequencies: &HashMap<u8, usize>) -> HuffmanTree {
    let mut heap = BinaryHeap::new();

    // Populate the heap with leaf nodes
    for (&value, &freq) in frequencies.iter() {
        heap.push(HuffmanTree::Leaf { value, freq });
    }

    // Build the Huffman tree by combining nodes until only one node is left
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let combined_freq = left.frequency() + right.frequency();
        let new_node = HuffmanTree::Node {
            freq: combined_freq,
            left: Box::new(left),
            right: Box::new(right),
        };
        heap.push(new_node);
    }
    // The last remaining node is the root of the Huffman tree
    heap.pop().unwrap()
}

fn build_huffman_codes(tree: &HuffmanTree, code: &mut HashMap<u8, Vec<Bit>>, current_code: Vec<Bit>) {
    match tree {
        HuffmanTree::Node { left, right, .. } => {
            let mut left_code = current_code.clone();
            left_code.push(Zero);
            build_huffman_codes(left, code, left_code);

            let mut right_code = current_code.clone();
            right_code.push(One);
            build_huffman_codes(right, code, right_code);
        }
        HuffmanTree::Leaf { value, .. } => {
            code.insert(*value, current_code);
        }
    }
}

fn encode_huffman(input: &[u8], codes: &HashMap<u8, Vec<Bit>>) -> BitVec {
    let mut bitvec = BitVec::new();
    for c in input {
        for bit in &codes[c]{
            bitvec.push(*bit);
        }
        
    }
    bitvec
}

pub fn encode(file: &Vec<u8>) -> (BitVec, HuffmanTree) {
    // Calculate character frequencies
    let mut frequencies: HashMap<u8, usize> = HashMap::new();
    for &value in file.iter() {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    // Build the Huffman tree
    let huffman_tree: HuffmanTree = build_huffman_tree(&frequencies);

    // Build Huffman codes
    let mut huffman_codes: HashMap<u8, Vec<Bit>> = HashMap::new();
    match &huffman_tree.clone() {
        HuffmanTree::Node { .. } => {build_huffman_codes(&huffman_tree, &mut huffman_codes, Vec::new());},
        HuffmanTree::Leaf { value, .. } => {{huffman_codes.insert(*value, vec![Zero]);}},
    }
    let mut sum_code_len = 0;
    for code in &huffman_codes {
        sum_code_len += code.1.len();
    }
    println!("mean code length: {:?}", sum_code_len as f64/ huffman_codes.len() as f64);
    // Encode form huffman code
    (encode_huffman(file, &huffman_codes), huffman_tree)
}

pub fn decode(encoded: BitVec, tree: &HuffmanTree) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current_node = tree;
    for bit in encoded {
        match bit {
            Zero => {
                if let HuffmanTree::Node { left, .. } = current_node {
                    current_node = left;
                }
            }
            One => {
                if let HuffmanTree::Node { right, .. } = current_node {
                    current_node = right;
                }
            }
        }

        if let HuffmanTree::Leaf { value, .. } = current_node {
            result.push(*value);
            current_node = tree; // Reset to the root for the next character
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encode_decode_test() {
        let file = std::fs::read("test_cases/test1.bin").unwrap();
        let (encoded, key) = encode(&file);
        println!("{:?}", &encoded);
        println!("{:?}", &key);
        let decoded = decode(encoded, &key);
        assert_eq!(file, decoded);
    }
}
