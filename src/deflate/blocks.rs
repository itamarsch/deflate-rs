use dynamic_huffman::read_dynamic_huffman;
use uncompressed::read_uncompressed;

use crate::bit_reader::BitReader;

use super::{
    huffman_tree::LiteralDistanceTrees,
    length_distance::{decode_length_distance, LengthDistance},
};

mod dynamic_huffman;
mod fixed_huffman;
mod uncompressed;

pub fn read_block(reader: &mut BitReader, buf: &mut Vec<u8>) -> bool {
    let bfinal = reader.read_bool();
    let block_type = reader.read_n_bits(2);
    match block_type {
        0b00 => {
            let data = read_uncompressed(reader);
            buf.extend_from_slice(data);
        }
        0b01 => {
            let trees = LiteralDistanceTrees::fixed_huffman();
            read_compressed_block(reader, buf, trees);
        }
        0b10 => {
            let trees: LiteralDistanceTrees = read_dynamic_huffman(reader);
            read_compressed_block(reader, buf, trees);
        }
        0b11 => unreachable!("Reserved"),
        _ => unreachable!("3 bits"),
    }

    bfinal
}

fn read_compressed_block(
    reader: &mut BitReader,
    buf: &mut Vec<u8>,
    LiteralDistanceTrees {
        literal_length,
        distance,
    }: LiteralDistanceTrees,
) {
    loop {
        let value = literal_length.decode_symbol(reader);
        match value.cmp(&256) {
            std::cmp::Ordering::Less => {
                buf.push(value as u8);
            }
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => {
                let LengthDistance { length, distance } =
                    decode_length_distance(reader, &distance, value);
                let source = buf.len() - distance;
                if length <= distance {
                    let range = source..source + length;
                    buf.extend_from_within(range);
                } else {
                    buf.reserve(length);
                    for i in 0..length {
                        let byte = buf[source + (i % distance)];
                        buf.push(byte);
                    }
                }
            }
        }
    }
}
