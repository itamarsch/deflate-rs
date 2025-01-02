use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use header::{FileType, TarHeader};
use nom::{bytes::streaming::take, IResult, InputIter};

mod header;

pub fn read_str_until_nul(len: usize) -> impl Fn(&[u8]) -> IResult<&[u8], &str> {
    move |s| {
        let (rest, field) = take(len)(s)?;
        let first_zero = field.position(|e| e == 0).unwrap_or(len);
        let (_, s) = take(first_zero)(field)?;

        Ok((rest, std::str::from_utf8(s).unwrap()))
    }
}

pub fn read_base_8_str_until_nul(len: usize) -> impl Fn(&[u8]) -> IResult<&[u8], u64> {
    move |s| {
        let (s, value) = read_str_until_nul(len)(s)?;
        let value = value.trim();
        Ok((
            s,
            if value.is_empty() {
                0
            } else {
                u64::from_str_radix(value, 8).unwrap()
            },
        ))
    }
}

pub fn write_file(
    data: &[u8],
    tar_header: &TarHeader,
    next_filename: &mut Option<&str>,
) -> io::Result<()> {
    let path = if let Some(filename) = next_filename {
        let path_buf = PathBuf::from(*filename);
        *next_filename = None;
        path_buf
    } else if tar_header.filename_prefix.is_empty() {
        tar_header.filename.to_path_buf()
    } else {
        Path::new(&tar_header.filename_prefix).join(tar_header.filename)
    };
    if let FileType::Directory = tar_header.file_type {
        fs::create_dir_all(&path)?;
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    match tar_header.file_type {
        FileType::NormalFile | FileType::ContiguousFile => {
            let mut file = File::create(&path)?;
            file.write_all(data)?;
        }
        _ => {
            eprintln!("Unsupported file type: {:?}", tar_header.file_type);
        }
    }

    Ok(())
    // Return success
}

pub fn inflate_tar(mut input: &[u8]) -> IResult<&[u8], ()> {
    let mut next_filename = None;

    while !input.is_empty() {
        let tar_header;
        ((input, tar_header)) = TarHeader::parse(input)?;

        let file_content;
        (input, file_content) = take(tar_header.file_size)(input)?;

        if let FileType::VendorSpecific('L') = tar_header.file_type {
            next_filename =
                Some(std::str::from_utf8(&file_content[..file_content.len() - 1]).unwrap());
        } else {
            write_file(file_content, &tar_header, &mut next_filename).unwrap();
        }

        println!(
            "{}/{} {:?}",
            tar_header.filename_prefix,
            tar_header.filename.display(),
            tar_header.file_type
        );

        (input, _) = take((512 - file_content.len() % 512) % 512)(input)?;

        let (_, next_two_chunks) = take(1024usize)(input)?;
        if next_two_chunks.iter().all(|e| *e == 0) {
            break;
        }
    }
    Ok((input, ()))
}
