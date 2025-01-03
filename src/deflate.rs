use blocks::read_block;
use indicatif::ProgressBar;

use crate::bit_reader::BitReader;

pub mod blocks;
mod huffman_dict;
pub mod huffman_tree;
pub mod length_distance;

pub fn read_deflate(compressed: &[u8], original_file_size: Option<usize>) -> (&[u8], Vec<u8>) {
    let mut reader = BitReader::new(compressed);
    let mut buf = Vec::with_capacity(original_file_size.unwrap_or(0));

    let bar = ProgressBar::new((compressed.len()) as u64);
    loop {
        let start_pos = reader.pos();

        let final_block = read_block(&mut reader, &mut buf);
        let end_pos = reader.pos();
        bar.inc((end_pos - start_pos) as u64 / 8);
        if final_block {
            break;
        }
    }
    bar.finish();
    let compressed_length = reader.bytes_read();
    (&compressed[compressed_length..], buf)
}
