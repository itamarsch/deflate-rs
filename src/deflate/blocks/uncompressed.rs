use std::ops::Not;

use crate::bit_reader::BitReader;

pub fn read_uncompressed<'a, 'b>(reader: &'b mut BitReader<'a>) -> &'a [u8] {
    reader.read_until_byte_boundry();
    let len = reader.read_u16();
    let nlen = reader.read_u16();
    assert_eq!(len as u16, (nlen as u16).not());
    let data = reader.read_bytes(len);
    data
}
