use std::{
    fs::File,
    io::{Read, Write},
};

use deflate::zlib::read_zlib;

fn main() {
    let mut file = File::open("z.zz").unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    let (_, buf) = read_zlib(&bytes[..]).unwrap();
    let mut file = File::create("decompressed").unwrap();
    file.write_all(&buf[..]).unwrap();
}
