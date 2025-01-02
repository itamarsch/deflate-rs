use crate::bit_reader::BitReader;

use super::huffman_tree::HuffmanTree;

pub struct LengthDistance {
    pub length: usize,
    pub distance: usize,
}

pub fn decode_length_distance(
    reader: &mut BitReader,
    distance_tree: &HuffmanTree,
    length_byte: u16,
) -> LengthDistance {
    let length_extra_bits = length_extra_bits(length_byte);
    let length_extra = reader.read_n_bits(length_extra_bits);
    let length = decode_length(length_byte as usize, length_extra);

    let distance_byte = distance_tree.decode_symbol(reader);
    let distance_extra_bits = distance_extra_bits(distance_byte);
    let distance_extra = reader.read_n_bits(distance_extra_bits);
    let distance = decode_distance(distance_byte as usize, distance_extra);
    LengthDistance { length, distance }
}

fn decode_length(length_byte: usize, extra: usize) -> usize {
    match length_byte {
        257..=264 => (length_byte - 257) + 3 + extra,
        265..=268 => (length_byte - 265) * 2 + 11 + extra,
        269..=272 => (length_byte - 269) * 4 + 19 + extra,
        273..=276 => (length_byte - 273) * 8 + 35 + extra,
        277..=280 => (length_byte - 277) * 16 + 67 + extra,
        281..=284 => (length_byte - 281) * 32 + 131 + extra,
        285 => 258,
        _ => panic!("invalid length"),
    }
}

fn decode_distance(distance_byte: usize, extra: usize) -> usize {
    match distance_byte {
        0..=3 => distance_byte + 1,
        4..=5 => (distance_byte - 4) * 2 + 5 + extra,
        6..=7 => (distance_byte - 6) * 4 + 9 + extra,
        8..=9 => (distance_byte - 8) * 8 + 17 + extra,
        10..=11 => (distance_byte - 10) * 16 + 33 + extra,
        12..=13 => (distance_byte - 12) * 32 + 65 + extra,
        14..=15 => (distance_byte - 14) * 64 + 129 + extra,
        16..=17 => (distance_byte - 16) * 128 + 257 + extra,
        18..=19 => (distance_byte - 18) * 256 + 513 + extra,
        20..=21 => (distance_byte - 20) * 512 + 1025 + extra,
        22..=23 => (distance_byte - 22) * 1024 + 2049 + extra,
        24..=25 => (distance_byte - 24) * 2048 + 4097 + extra,
        26..=27 => (distance_byte - 26) * 4096 + 8193 + extra,
        28..=29 => (distance_byte - 28) * 8192 + 16385 + extra,
        _ => panic!("Invalid distance"),
    }
}

fn length_extra_bits(length_byte: u16) -> usize {
    assert!(257 <= length_byte && length_byte <= 285);
    let length_byte = length_byte - 257;
    if length_byte == 28 || length_byte < 4 {
        0
    } else {
        ((length_byte - 4) / 4) as usize
    }
}

fn distance_extra_bits(distance_byte: u16) -> usize {
    assert!(distance_byte <= 29);
    if distance_byte < 2 {
        0
    } else {
        ((distance_byte - 2) / 2) as usize
    }
}
