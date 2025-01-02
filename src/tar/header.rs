use std::path::Path;

use nom::{
    bytes::complete::{tag, take},
    IResult, Parser,
};

use super::{read_base_8_str_until_nul, read_str_until_nul};

#[derive(Debug)]
pub struct TarHeader<'a> {
    pub filename: &'a Path,
    pub file_mode: u64,
    pub owner_id: u64,
    pub group_id: u64,
    pub file_size: u64,
    pub last_modification: u64,
    pub header_checksum: u64,
    pub file_type: FileType,
    pub name_of_linked: &'a str,
    pub owner: &'a str,
    pub owner_group: &'a str,
    pub device_major: &'a str,
    pub device_minor: &'a str,
    pub filename_prefix: &'a str,
}

impl TarHeader<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TarHeader> {
        let (input, filename) = read_str_until_nul(100).map(Path::new).parse(input)?;
        let (input, file_mode) = read_base_8_str_until_nul(8)(input)?;
        let (input, owner_id) = read_base_8_str_until_nul(8)(input)?;
        let (input, group_id) = read_base_8_str_until_nul(8)(input)?;
        let (input, file_size) = read_base_8_str_until_nul(12)(input)?;
        let (input, last_modification) = read_base_8_str_until_nul(12)(input)?;
        let (input, header_checksum) = read_base_8_str_until_nul(8)(input)?;
        let (input, file_type) = take(1usize)
            .map(|e: &[u8]| e[0] as char)
            .map(FileType::parse)
            .parse(input)?;
        let (input, name_of_linked) = read_str_until_nul(100)(input)?;
        let (input, _) = tag(b"ustar ")(input)?;
        let (input, _) = take(2usize)(input)?;
        let (input, owner) = read_str_until_nul(32)(input)?;
        let (input, owner_group) = read_str_until_nul(32)(input)?;
        let (input, device_major) = read_str_until_nul(8)(input)?;
        let (input, device_minor) = read_str_until_nul(8)(input)?;
        let (input, filename_prefix) = read_str_until_nul(155)(input)?;
        let (input, _end) = take(12usize)(input)?;

        Ok((
            input,
            TarHeader {
                filename,
                file_mode,
                owner_id,
                group_id,
                file_size,
                last_modification,
                header_checksum,
                file_type,
                name_of_linked,
                owner,
                owner_group,
                device_major,
                device_minor,
                filename_prefix,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    NormalFile,
    HardLink,
    SymbolicLink,
    CharacterSpecial,
    BlockSpecial,
    Directory,
    FIFO,
    ContiguousFile,
    GlobalExtendedHeader,
    ExtendedHeader,
    VendorSpecific(char),
    Reserved,
}

impl FileType {
    pub fn parse(byte: char) -> Self {
        match byte {
            '0' | '\0' => FileType::NormalFile,
            '1' => FileType::HardLink,
            '2' => FileType::SymbolicLink,
            '3' => FileType::CharacterSpecial,
            '4' => FileType::BlockSpecial,
            '5' => FileType::Directory,
            '6' => FileType::FIFO,
            '7' => FileType::ContiguousFile,
            'g' => FileType::GlobalExtendedHeader,
            'x' => FileType::ExtendedHeader,
            c if c.is_ascii_alphabetic() && c.is_uppercase() => FileType::VendorSpecific(c),
            _ => FileType::Reserved,
        }
    }
}
