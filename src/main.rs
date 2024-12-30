use std::{cmp::min, fs::File, io::Read};

use nom::{bytes::complete::take, IResult};

struct BitReader<'a> {
    pub data: &'a [u8],
    pub bit_position: usize,
}

impl BitReader<'_> {
    fn new<'a>(data: &'a [u8]) -> BitReader<'a> {
        BitReader {
            data,
            bit_position: 0,
        }
    }

    fn read_n_bits(&mut self, mut n: usize) -> usize {
        let mut result: usize = 0;
        let mut shift = 0;
        while n > 0 {
            let byte_index = self.bit_position / 8;
            let bit_index = self.bit_position % 8;
            let bits_in_this_byte = min(n, 8 - bit_index);

            let mask = (1 << bits_in_this_byte) - 1;
            let val = (self.data[byte_index] as usize >> bit_index) & mask;

            result |= val << shift;
            shift += bits_in_this_byte;

            self.bit_position += bits_in_this_byte;
            n -= bits_in_this_byte
        }

        result
    }

    fn read_bool(&mut self) -> bool {
        self.read_n_bits(1) == 1
    }
}

fn main() {
    let mut file = File::open("compressed").unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let (_, buf) = read_deflate(&bytes[..]).unwrap();
    println!("{:?}", std::str::from_utf8(&buf));
}

fn read_deflate(rest: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, _) = take(2usize)(rest)?;
    let (_, rest) = take(rest.len() - 4)(rest)?;
    let mut reader = BitReader::new(rest);
    let mut buf = Vec::new();

    loop {
        let final_block = read_block(&mut reader, &mut buf);
        if final_block {
            break;
        }
    }

    Ok((rest, buf))
}

fn read_block(reader: &mut BitReader, buf: &mut Vec<u8>) -> bool {
    let bfinal = reader.read_bool();
    let block_type = reader.read_n_bits(2);
    match block_type {
        0 => {
            unimplemented!("Uncompressed");
        }
        0b01 => {
            read_fixed(reader, buf);
        }
        0b10 => {
            unimplemented!("Dynamic")
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

fn read_fixed(reader: &mut BitReader, buf: &mut Vec<u8>) {
    let (literal_tree, distance_tree) = fixed_huffman_tree();
    loop {
        let value = decode_symbol(reader, &literal_tree);
        match value.cmp(&256) {
            std::cmp::Ordering::Less => {
                buf.push(value as u8);
            }
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => {
                let (length, distance) = decode_length_distance(reader, &distance_tree, value);
                if length <= distance {
                    let source = buf.len() - distance;
                    let range = source..source + length;
                    buf.extend_from_within(range);
                } else {
                    todo!()
                }
            }
        }
    }
}

fn decode_length_distance(
    reader: &mut BitReader,
    distance_tree: &Vec<HuffmanSymbol>,
    length_byte: u16,
) -> (usize, usize) {
    let length_extra_bits = length_extra_bits(length_byte);
    let length_extra = reader.read_n_bits(length_extra_bits);
    let length = decode_length(length_byte as usize, length_extra);

    let distance_byte = decode_symbol(reader, &distance_tree);
    let distance_extra_bits = distance_extra_bits(distance_byte);
    let distance_extra = reader.read_n_bits(distance_extra_bits);
    let distance = decode_distance(distance_byte as usize, distance_extra);

    (length, distance)
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
    match length_byte {
        257..=264 => 0,
        265..=268 => 1,
        269..=272 => 2,
        273..=276 => 3,
        277..=280 => 4,
        281..=284 => 5,
        285 => 0,
        _ => panic!("Invalid length value"),
    }
}

fn distance_extra_bits(distance_byte: u16) -> usize {
    if distance_byte < 2 {
        0
    } else {
        ((distance_byte - 2) / 4) as usize
    }
}

fn fixed_huffman_tree() -> (Vec<HuffmanSymbol>, Vec<HuffmanSymbol>) {
    let mut literals = [0; 288];
    let mut distances = [0; 32];

    for i in 0..144 {
        literals[i] = 8;
    }

    for i in 144..256 {
        literals[i] = 9;
    }

    for i in 256..280 {
        literals[i] = 7;
    }

    for i in 280..288 {
        literals[i] = 8;
    }

    for i in 0..distances.len() {
        distances[i] = 5;
    }
    let literal_tree = build_huffman_tree(&literals);
    let distance_tree = build_huffman_tree(&distances);
    (literal_tree, distance_tree)
}

#[derive(Clone, Copy, Debug)]
struct HuffmanSymbol {
    symbol: u16,
    value: u16,
    length: u16,
}

fn build_huffman_tree(code_lengths: &[u8]) -> Vec<HuffmanSymbol> {
    let max_length = code_lengths.iter().max().unwrap();
    let mut bl_count = vec![0; *max_length as usize + 1];
    for &code_length in code_lengths {
        if code_length == 0 {
            continue;
        }
        bl_count[code_length as usize] += 1;
    }

    let mut huffman_codes = vec![0; *max_length as usize + 1];
    let mut code: usize = 0;
    for bits in 1..=*max_length {
        code = (code + bl_count[bits as usize - 1]) << 1;
        huffman_codes[bits as usize] = code;
    }

    let mut huffman_tree = Vec::new();

    for (symbol, &length) in code_lengths.iter().enumerate() {
        if length > 0 {
            huffman_tree.push(HuffmanSymbol {
                symbol: huffman_codes[length as usize] as u16,
                value: symbol as u16,
                length: length as u16,
            });
            huffman_codes[length as usize] += 1;
        }
    }
    huffman_tree
}

// const CODE_LENGTH_ORDER: [u8; 19] = [
//     16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 4, 3, 2, 1, 12, 13, 14, 15, 11,
// ];

//     let hlit = reader.read_n_bits(5) + 257;
//     let hdist = reader.read_n_bits(5) + 1;
//     let hclen = reader.read_n_bits(4) + 4;
//     println!("{} {:2b} {} {} {}", bfinal, block_type, hlit, hdist, hclen);
//     let mut code_length_code_lengths = [0u8; 19];
//     for i in 0..hclen {
//         let len = reader.read_n_bits(3) as u8; // each is stored in 3 bits
//         code_length_code_lengths[CODE_LENGTH_ORDER[i as usize] as usize] = len;
//     }
//     println!("{:?}", code_length_code_lengths);

//     let code_length_tree = build_huffman_tree(&code_length_code_lengths);

//     for i in (0..hlit) {
//         let symbol = decode_symbol(reader, &code_length_tree);
//     }

fn decode_symbol(reader: &mut BitReader, huffman_tree: &Vec<HuffmanSymbol>) -> u16 {
    // println!("{:#?}", huffman_tree.iter().find(|n|n.value == 3));
    let mut code: u16 = 0;

    for bit_count in 1..=16 {
        // Maximum possible code length (adjust as needed)
        code <<= 1;
        code |= reader.read_n_bits(1) as u16;

        // Search for a matching symbol
        if let Some(symbol) = huffman_tree
            .iter()
            .find(|node| node.length == bit_count && node.symbol == code)
        {
            return symbol.value;
        }
    }

    panic!("No symbol found")
}
