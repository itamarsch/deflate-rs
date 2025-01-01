use std::time::{Duration, SystemTime, UNIX_EPOCH};

use nom::{
    bytes::complete::{tag, take_until},
    number::complete::{le_u32, u8},
    IResult,
};

use crate::deflate::read_deflate;

pub struct Gzip<'a> {
    pub filename: &'a str,
    pub data: Vec<u8>,
    pub mtime: SystemTime,
}

pub fn read_gzip(input: &[u8]) -> IResult<&[u8], Gzip> {
    const GZIP_HEADER: &[u8] = &[31, 139];
    let (input, _) = tag(GZIP_HEADER)(input)?;
    let (input, cm) = u8(input)?;
    assert!(cm == 8);
    let (input, flg) = u8(input)?;
    let _ftxt = flg & 1 == 1;
    let fhcrc = (flg >> 1) & 1 == 1;
    let fextra = (flg >> 2) & 1 == 1;
    let fname = (flg >> 3) & 1 == 1;
    let fcomment = (flg >> 4) & 1 == 1;
    let (input, mtime) = le_u32(input)?;
    let mtime = UNIX_EPOCH + Duration::from_secs(mtime as u64);
    let (input, _xfl) = u8(input)?;
    let (input, _os) = u8(input)?;

    let (input, ()) = if fextra { todo!() } else { (input, ()) };

    let (input, name) = if fname {
        let (input, name) = take_until(&[0][..])(input)?;
        let (input, _) = u8(input)?;
        (input, name)
    } else {
        (input, &[][..])
    };

    let (input, ()) = if fcomment { todo!() } else { (input, ()) };
    let (input, ()) = if fhcrc { todo!() } else { (input, ()) };

    let (input, decompressed) = read_deflate(input);
    assert!(input.len() == 8);
    let (input, crc32) = le_u32(input)?;

    let calculated_crc = crc32fast::hash(&decompressed);
    let (input, isize) = le_u32(input)?;

    assert_eq!(calculated_crc, crc32);
    assert!(isize as usize == decompressed.len() % (2 << 32));

    Ok((
        input,
        Gzip {
            data: decompressed,
            filename: std::str::from_utf8(name).unwrap(),
            mtime,
        },
    ))
}
