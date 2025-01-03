use std::{collections::HashMap, hash::Hash};

use crate::bit_reader::BitReader;
use fxhash::FxBuildHasher;

pub struct HuffmanTree(HashMap<HuffmanCode, u16, FxBuildHasher>);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HuffmanCode {
    code: u16,
    length: u16,
}

pub struct LiteralDistanceTrees {
    pub literal_length: HuffmanTree,
    pub distance: HuffmanTree,
}

impl HuffmanTree {
    pub fn new<const T: usize>(code_lengths: &[u8]) -> Self {
        let max_length = code_lengths.iter().max().unwrap();
        let mut codes_per_length = [0; T];
        for &code_length in code_lengths {
            if code_length == 0 {
                continue;
            }
            codes_per_length[code_length as usize] += 1;
        }

        let mut huffman_codes = [0; T];
        let mut code: usize = 0;
        for bits in 1..=*max_length {
            code = (code + codes_per_length[bits as usize - 1]) << 1;
            huffman_codes[bits as usize] = code;
        }

        let mut huffman_tree = HashMap::default();

        for (value, &length) in code_lengths.iter().enumerate() {
            if length > 0 {
                huffman_tree.insert(
                    HuffmanCode {
                        code: huffman_codes[length as usize] as u16,
                        length: length as u16,
                    },
                    value as u16,
                );
                huffman_codes[length as usize] += 1;
            }
        }

        Self(huffman_tree)
    }

    pub fn decode_symbol(&self, reader: &mut BitReader) -> u16 {
        // println!("{:#?}", huffman_tree.iter().find(|n|n.value == 3));
        let mut code: u16 = 0;

        for bit_count in 1..=16 {
            // Maximum possible code length (adjust as needed)
            code <<= 1;
            code |= reader.read_n_bits(1) as u16;

            // Search for a matching symbol
            if let Some(value) = self.0.get(&HuffmanCode {
                code,
                length: bit_count,
            }) {
                return *value;
            }
        }

        panic!("No symbol found")
    }
}
