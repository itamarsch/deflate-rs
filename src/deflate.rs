use blocks::read_block;

use crate::bit_reader::BitReader;

pub mod blocks;
pub mod huffman_tree;
pub mod length_distance;

pub fn read_deflate(compressed: &[u8]) -> (&[u8], Vec<u8>) {
    let mut reader = BitReader::new(compressed);
    let mut buf = Vec::new();

    loop {
        let final_block = read_block(&mut reader, &mut buf);
        if final_block {
            break;
        }
    }
    let compressed_length = reader.bytes_read();
    (&compressed[compressed_length..], buf)
}
