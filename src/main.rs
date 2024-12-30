use std::{fs::File, io::Read};

use deflate::{bit_reader::BitReader, deflate::blocks::read_block};
use nom::{bytes::complete::take, IResult};

fn main() {
    let mut file = File::open("compressed").unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let (_, buf) = read_zlib(&bytes[..]).unwrap();
    println!("{:?}", std::str::from_utf8(&buf));
}

fn read_zlib(rest: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, _header) = take(2usize)(rest)?;
    let (_footer, rest) = take(rest.len() - 4)(rest)?;
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
