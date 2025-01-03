use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use clap::{command, Parser};
use deflate::{gzip::read_gzip, tar::inflate_tar};
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

fn main() -> io::Result<()> {
    let args = DecompressArgs::parse();

    let file = read_file_bytes(&args.input)?;

    if args.input.extension().is_some_and(|ext| ext == "gz")
        && args
            .input
            .file_stem()
            .is_some_and(|stem| stem.to_str().is_some_and(|stem| stem.ends_with(".tar")))
    {
        let (_, gzip) = read_gzip(&file).unwrap();

        // inflate_tar(&gzip.data).unwrap();
    }

    // let (_, decompressed) = read_zlib(&file, args.dict.as_ref().map(|e| e.as_str())).unwrap();

    Ok(())
}
