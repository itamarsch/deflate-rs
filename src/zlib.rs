use adler::Adler32;
use nom::{bytes::complete::take, number::complete::u8, IResult};

use crate::deflate::read_deflate;

pub fn read_zlib(rest: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, ()) = read_header(rest)?;
    let (footer, rest) = take(rest.len() - 4)(rest)?;

    let buf = read_deflate(rest);

    check_adler(footer.try_into().unwrap(), &buf);

    Ok((rest, buf))
}

fn read_header(header: &[u8]) -> IResult<&[u8], ()> {
    let (header, cmf) = u8(header)?;
    let compression_method = cmf & 0x0f;
    assert_eq!(compression_method, 8, "Invalid compression method!");
    let compression_info = ((cmf & 0xf0) >> 4) as u32;
    assert_eq!(2u32.pow(compression_info + 8), 32768);

    let (header, flg) = u8(header)?;

    let check = ((cmf as u16) << 8) | flg as u16;

    let _fdict = flg & 0b00100000 >> 5;
    let header = if _fdict == 1 {
        let (header, _dict_id) = take(4usize)(header)?;

        header
    } else {
        header
    };

    let _compression_level = flg & 0b11000000 >> 6;

    assert_eq!(check % 31, 0, "Zlib header validity check");
    Ok((header, ()))
}

fn check_adler(checksum: [u8; 4], bytes: &[u8]) {
    let mut adler = Adler32::new();
    adler.write_slice(bytes);
    let calculated = adler.checksum();

    let checksum = u32::from_be_bytes(checksum);
    assert_eq!(calculated, checksum, "Checksum mismatch!");
}
