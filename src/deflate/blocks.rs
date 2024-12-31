use dynamic_huffman::read_dynamic_huffman;
use fixed_huffman::fixed_huffman_tree;

use crate::{bit_reader::BitReader, huffman_tree::HuffmanTree};

use super::length_distance::decode_length_distance;

mod dynamic_huffman;
mod fixed_huffman;

pub fn read_block(reader: &mut BitReader, buf: &mut Vec<u8>) -> bool {
    let bfinal = reader.read_bool();
    let block_type = reader.read_n_bits(2);
    match block_type {
        0 => {
            unimplemented!("Uncompressed");
        }
        0b01 => {
            let (literal, distance) = fixed_huffman_tree();
            read_compressed_block(reader, buf, literal, distance);
        }
        0b10 => {
            let (literal, distance) = read_dynamic_huffman(reader);
            read_compressed_block(reader, buf, literal, distance);
        }
        0b11 => {
            unreachable!("Reserved")
        }
        _ => {
            unreachable!("3 bits")
        }
    }

    bfinal
}

fn read_compressed_block(
    reader: &mut BitReader,
    buf: &mut Vec<u8>,
    literal_tree: HuffmanTree,
    distance_tree: HuffmanTree,
) {
    loop {
        let value = literal_tree.decode_symbol(reader);
        match value.cmp(&256) {
            std::cmp::Ordering::Less => {
                buf.push(value as u8);
            }
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => {
                let (length, distance) = decode_length_distance(reader, &distance_tree, value);
                let source = buf.len() - distance;
                if length <= distance {
                    let range = source..source + length;
                    buf.extend_from_within(range);
                } else {
                    for i in 0..length {
                        let byte = buf[source + (i % distance)];
                        buf.push(byte);
                    }
                }
            }
        }
    }
}
