use crate::{bit_reader::BitReader, huffman_tree::HuffmanTree};

const CODE_LENGTH_ORDER: [u8; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

pub fn read_dynamic_huffman(reader: &mut BitReader) -> (HuffmanTree, HuffmanTree) {
    let hlit = reader.read_n_bits(5) + 257;
    let hdist = reader.read_n_bits(5) + 1;
    let hclen = reader.read_n_bits(4) + 4;
    let mut code_length_code_lengths = [0u8; 19];
    for i in 0..hclen {
        let len = reader.read_n_bits(3) as u8; // each is stored in 3 bits
        code_length_code_lengths[CODE_LENGTH_ORDER[i as usize] as usize] = len;
    }

    let code_length_tree = HuffmanTree::new(&code_length_code_lengths);
    let literal_table = build_dynamic_huffman::<288>(reader, hlit, &code_length_tree);
    let dist_tables = build_dynamic_huffman::<32>(reader, hdist, &code_length_tree);
    (literal_table, dist_tables)
}

fn build_dynamic_huffman<const T: usize>(
    reader: &mut BitReader,
    count: usize,
    lengths: &HuffmanTree,
) -> HuffmanTree {
    let mut index = 0;
    let mut symbols = [0; T];
    while index < count {
        let value = lengths.decode_symbol(reader);
        let (value, times) = match value {
            0..=15 => (value as u8, 1),
            16 => {
                let times = reader.read_n_bits(2) + 3;
                (symbols[index - 1], times)
            }
            17 => {
                let times = reader.read_n_bits(3) + 3;
                (0, times)
            }
            18 => {
                let times = reader.read_n_bits(7) + 11;
                (0, times)
            }
            _ => {
                unreachable!()
            }
        };
        for _ in 0..times {
            symbols[index] = value;
            index += 1;
        }
    }
    HuffmanTree::new(&symbols[..])
}
