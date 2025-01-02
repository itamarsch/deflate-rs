use adler::Adler32;
use nom::{bytes::complete::take, number::complete::u8, IResult};

use crate::deflate::read_deflate;

pub fn read_zlib<'a, 'b>(rest: &'a [u8], dict: Option<&'b str>) -> IResult<&'a [u8], Vec<u8>> {
    let (rest, ()) = read_header(rest, dict)?;

    let (footer, buf) = read_deflate(rest);
    assert!(footer.len() == 4);

    check_adler(footer.try_into().unwrap(), &buf);

    Ok((rest, buf))
}

fn read_header<'a, 'b>(header: &'a [u8], dict: Option<&'b str>) -> IResult<&'a [u8], ()> {
    let (header, cmf) = u8(header)?;
    let compression_method = cmf & 0x0f;
    assert_eq!(compression_method, 8, "Invalid compression method!");
    let compression_info = ((cmf & 0xf0) >> 4) as u32;
    assert_eq!(2u32.pow(compression_info + 8), 32768);

    let (header, flg) = u8(header)?;

    let check = ((cmf as u16) << 8) | flg as u16;

    let fdict = (flg >> 5) & 0x01;

    let header = if fdict == 1 {
        let (header, _dict_id) = take(4usize)(header)?;
        if let Some(dict) = dict {
            check_adler(_dict_id.try_into().unwrap(), dict.as_bytes());
        } else {
            println!("File has a dict ID but no dict was passed!")
        };

        header
    } else {
        if dict.is_some() {
            println!("Ignoring dictionary passed, File has no dictionary")
        }
        header
    };

    let _compression_level = (flg >> 6) & 0x11;

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
