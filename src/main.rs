use std::{
    fs::File,
    io::{Read, Write},
};

use deflate::{bit_reader::BitReader, deflate::blocks::read_block};
use nom::{bytes::complete::take, IResult};

fn main() {
    let mut file = File::open("uncompressed.bin").unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let (_, buf) = read_zlib(&bytes[..]).unwrap();
    let mut file = File::create("decompressed.json").unwrap();
    file.write_all(&buf[..]).unwrap();
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
