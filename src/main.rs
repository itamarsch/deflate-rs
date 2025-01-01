use std::{
    fs::File,
    io::{self, Read, Write},
    time::SystemTime,
};

use clap::{command, Parser};
use deflate::gzip::{read_gzip, Gzip};
/// CLI Parser Example
#[derive(Parser, Debug)]
#[command(about = "Decompress a zlib file")]
struct DecompressArgs {
    /// Specifies the input file
    input: String,
}

fn read_file_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

fn write_output(filename: &str, mtime: SystemTime, decompressed: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.set_modified(mtime)?;
    file.write_all(decompressed)
}

fn main() -> io::Result<()> {
    let args = DecompressArgs::parse();

    let file = read_file_bytes(&args.input)?;

    let (
        _,
        Gzip {
            filename,
            data,
            mtime,
        },
    ) = read_gzip(&file).unwrap();

    // let (_, decompressed) = read_zlib(&file, args.dict.as_ref().map(|e| e.as_str())).unwrap();

    write_output(filename, mtime, &data)?;

    Ok(())
}
