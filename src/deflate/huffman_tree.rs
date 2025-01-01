use crate::bit_reader::BitReader;

pub struct HuffmanTree(Vec<HuffmanSymbol>);

#[derive(Clone, Copy, Debug)]
pub struct HuffmanSymbol {
    symbol: u16,
    value: u16,
    length: u16,
}

pub struct LiteralDistanceTrees {
    pub literal_length: HuffmanTree,
    pub distance: HuffmanTree,
}

impl HuffmanTree {
    pub fn new(code_lengths: &[u8]) -> Self {
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
            if let Some(symbol) = self
                .0
                .iter()
                .find(|node| node.length == bit_count && node.symbol == code)
            {
                return symbol.value;
            }
        }

        panic!("No symbol found")
    }
}
