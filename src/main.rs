use std::{
    fs::File,
    io::{self, Read, Write},
};

use clap::{command, Parser};
use deflate::zlib::read_zlib;
/// CLI Parser Example
#[derive(Parser, Debug)]
#[command(about = "Decompress a zlib file")]
struct DecompressArgs {
    /// Specifies the input file
    input: String,

    /// A sequence of bytes. The adler checksum is calculated on this sequencs and compared to the zlib DictID (Read rfc 1950)
    #[arg(short, long)]
    dict: Option<String>,

    /// Specifies the output file
    #[arg(short = 'o', long = "output")]
    output: String,
}

fn read_file_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

fn write_output(filename: &str, decompressed: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(decompressed)
}

fn main() -> io::Result<()> {
    let args = DecompressArgs::parse();

    let file = read_file_bytes(&args.input)?;

    let (_, decompressed) = read_zlib(&file, args.dict.as_ref().map(|e| e.as_str())).unwrap();

    write_output(&args.output, &decompressed)?;

    Ok(())
}
