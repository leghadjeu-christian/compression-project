use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    marker::PhantomData,
    path::Path,
};

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
    pub input: String,

    /// Output file
    pub output: String,

    /// Use Run-Length Encoding
    #[arg(long, conflicts_with = "lz")]
    pub rle: bool,

    /// Use Lempel-Ziv Compression (not implemented)
    #[arg(long, conflicts_with = "rle")]
    pub lz: bool,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Code<T, U> {
    pub offset: U,
    pub length: U,
    pub literal: T,
}


use bincode::{deserialize_from, serialize_into};

/// Writes a vector of Code structs to a file
pub fn store_codes_to_file<T, U>(
    codes: &Vec<Code<T, U>>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize,
    U: serde::Serialize,
{
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serialize_into(writer, codes)?;
    Ok(())
}

/// Loads a vector of Code structs from a file
pub fn load_codes_from_file<T, U>(path: &str) -> Result<Vec<Code<T, U>>, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
    U: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let codes: Vec<Code<T, U>> = deserialize_from(reader)?;
    Ok(codes)
}
