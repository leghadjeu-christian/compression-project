mod custom;
mod lz77;
mod rle;

use custom::{load_codes_from_file, store_codes_to_file};
use clap::Parser;
use custom::{AlgoArgs, Cli, Commands};
use lz77::{compress_lz, decode, from_string, to_string};
use rle::{compress_rle, decompress_rle};
use std::{
    fs::{self, File},
    io::Read, path::{Path, PathBuf},
};

fn file_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return buffer;
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress(args) => {
            let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
            if args.rle {
                let output = compress_rle(&input);
                fs::write(&args.output, output).expect("Failed to write output file");
            } else if args.lz {
                let input = file_bytes(&args.input);
               let encoded = compress_lz::<u8, u8>(&input);
               println!("{:#?}", encoded);
               let _ = store_codes_to_file(&encoded, &args.output);
            } else {
                panic!("Please specify an algorithm with --rle or --lz");
            };
        }
        Commands::Decompress(args) => {
            let output = if args.rle {
                let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
                decompress_rle(&input)
            } else if args.lz {
                let input = Path::new(&args.input);
                let encoded: Vec<custom::Code<u8, u8>> = load_codes_from_file(&input.to_str().unwrap()).unwrap();
                decode(&encoded).iter().map(|f | f.to_string()).collect()
            } else {
                panic!("Please specify an algorithm with --rle or --lz");
            };
            fs::write(&args.output, output).expect("Failed to write output file");
        }
    }
}
