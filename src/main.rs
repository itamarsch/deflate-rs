use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use clap::{command, Parser};
use deflate::gzip::read_gzip;
/// CLI Parser Example
#[derive(Parser, Debug)]
#[command(about = "Decompress a zlib file")]
struct DecompressArgs {
    /// Specifies the input file
    input: PathBuf,
}

fn read_file_bytes(filename: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}

fn write_output(filename: &Path, mtime: SystemTime, decompressed: &[u8]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.set_modified(mtime)?;
    file.write_all(decompressed)
}

fn main() -> io::Result<()> {
    let args = DecompressArgs::parse();

    let file = read_file_bytes(&args.input)?;

    if args.input.extension().is_some_and(|ext| ext == "gz") {
        let (_, gzip) = read_gzip(&file).unwrap();

        write_output(gzip.filename, gzip.mtime, &gzip.data)?;
    }

    // let (_, decompressed) = read_zlib(&file, args.dict.as_ref().map(|e| e.as_str())).unwrap();

    Ok(())
}
