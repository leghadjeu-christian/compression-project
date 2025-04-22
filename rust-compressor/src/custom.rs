use clap::{Args, Parser, Subcommand};


/// A CLI tool to compress or decompress files using RLE or LZ (only RLE implemented here)
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compress a file
    Compress(AlgoArgs),

    /// Decompress a
    Decompress(AlgoArgs),
}

#[derive(Args)]
pub struct AlgoArgs {
    /// Input file
   pub  input: String,

    /// Output file
    pub output: String,

    /// Use Run-Length Encoding
    #[arg(long, conflicts_with = "lz")]
    pub rle: bool,

    /// Use Lempel-Ziv Compression (not implemented)
    #[arg(long, conflicts_with = "rle")]
    pub lz: bool,
}
