mod custom;
mod lz77;
mod rle;

use clap::Parser;
use custom::{AlgoArgs, Cli, Commands};
use rle::{compress_rle, decompress_rle};
use std::fs;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress(args) => {
            let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
            let output = if args.rle {
                compress_rle(&input)
            } else if args.lz {
                panic!("LZ compression not implemented");
            } else {
                panic!("Please specify an algorithm with --rle or --lz");
            };
            fs::write(&args.output, output).expect("Failed to write output file");
        }
        Commands::Decompress(args) => {
            let input = std::fs::read_to_string(&args.input).expect("Failed to read input file");
            let output = if args.rle {
                decompress_rle(&input)
            } else if args.lz {
                panic!("LZ decompression not implemented");
            } else {
                panic!("Please specify an algorithm with --rle or --lz");
            };
            fs::write(&args.output, output).expect("Failed to write output file");
        }
    }
}
