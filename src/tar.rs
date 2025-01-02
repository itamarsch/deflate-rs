use nom::{
    bytes::{complete::tag, streaming::take},
    IResult, InputIter,
};

pub fn read_str_until_nul(s: &[u8], len: usize) -> IResult<&[u8], &str> {
    let (rest, field) = take(len)(s)?;
    let first_zero = field.position(|e| e == 0).unwrap_or(len);
    let (_, s) = take(first_zero)(field)?;

    // assert!(a.iter().all(|z| *z == 0), "{:?}", a);
    Ok((rest, std::str::from_utf8(s).unwrap()))
}

pub fn read_tar(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, filename) = read_str_until_nul(input, 100)?;
    let (input, file_mode) = read_str_until_nul(input, 8)?;
    let (input, owner_id) = read_str_until_nul(input, 8)?;
    let (input, group_id) = read_str_until_nul(input, 8)?;
    let (input, file_size) = read_str_until_nul(input, 12)?;
    let (input, last_modification) = read_str_until_nul(input, 12)?;
    let (input, header_chcksum) = read_str_until_nul(input, 8)?;
    let (input, file_type) = read_str_until_nul(input, 1)?;
    let (input, name_of_linked) = read_str_until_nul(input, 100)?;
    let (input, _) = tag(b"ustar ")(input)?;
    let (input, _) = take(2usize)(input)?;
    let (input, owner) = read_str_until_nul(input, 32)?;
    let (input, owmer_group) = read_str_until_nul(input, 32)?;
    let (input, device_major) = read_str_until_nul(input, 8)?;
    let (input, device_minor) = read_str_until_nul(input, 8)?;
    let (input, filname_prefix) = read_str_until_nul(input, 155)?;
    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        filename,
        file_mode,
        owner_id,
        group_id,
        file_size,
        last_modification,
        header_chcksum,
        file_type,
        name_of_linked,
        owner,
        owmer_group,
        device_major,
        device_minor,
        filname_prefix
    );

    Ok((input, ()))
}
