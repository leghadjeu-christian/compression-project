// mod lz77;

// #[derive(Debug)]
// struct Token {
//     offset: usize,
//     length: usize,
//     next_char: char,
// }

// fn lz77_compress(input: &str, window_size: usize) -> Vec<Token> {
//     let chars: Vec<char> = input.chars().collect();
//     let mut tokens = Vec::new();
//     let mut pos = 0;

//     while pos < chars.len() {
//         let start = if pos < window_size {
//             0
//         } else {
//             pos - window_size
//         };
//         let mut match_len = 0;
//         let mut match_offset = 0;

//         for i in start..pos {
//             let mut length = 0;
//             while pos + length < chars.len() && chars[i + length] == chars[pos + length] {
//                 length += 1;
//                 if i + length >= pos {
//                     break;
//                 }
//             }

//             if length > match_len {
//                 match_len = length;
//                 match_offset = pos - i;
//             }
//         }

//         let next_char = if pos + match_len < chars.len() {
//             chars[pos + match_len]
//         } else {
//             '\0' // end marker
//         };

//         tokens.push(Token {
//             offset: match_offset,
//             length: match_len,
//             next_char,
//         });

//         pos += match_len + 1;
//     }

//     tokens
// }

// use std::mem;

// use lz77::LZ77Algorithm;
// fn main() {
//     let lz77 = LZ77Algorithm::new();

//     // input to compress
//     let argv: Vec<String> = std::env::args().collect();
//     let input = argv.get(1).unwrap();

//     // encode input
//     let encoded_result = lz77.encode(input);

//     println!("Result : {:?}", encoded_result);
//     // decode input
//     let decoded_result = lz77.decode(&encoded_result);

//     // check if decoded result is equal to input
//     println!(
//         "Encoded: {} | Decoded: {} | Input: {}",
//         encoded_result.len(),
//         decoded_result.len(),
//         input.len()
//     );
//     println!("> Encoded result: {:?}", encoded_result);
//     println!("> Decoded result: {:?}", decoded_result);

//     println!("Encoded bytes: {}", mem::size_of_val(&encoded_result));
//     println!("Decoded bytes: {}", mem::size_of_val(&decoded_result));
// }

mod custom;
mod rle;
mod lz77;

use clap::Parser;
use custom::{AlgoArgs, Cli, Commands};
use rle::{compress_rle, decompress_rle};
use std::fs;

#[derive(Debug)]
struct Token {
    offset: usize,
    length: usize,
    next_char: char,
}

fn get_match(search_buffer: &str, lookahead_buffer: &str) -> (usize, usize, String) {
    let mut max_length = 0;
    let mut offset = 0;

    for i in 0..search_buffer.len() {
        let mut length = 0;

        while i + length < search_buffer.len()
            && length < lookahead_buffer.len()
            && search_buffer[i + length..].starts_with(&lookahead_buffer[..length + 1])
        {
            length += 1;
        }

        if length > max_length {
            max_length = length;
            offset = search_buffer.len() - i;
        }
    }

    let matched = lookahead_buffer
        .chars()
        .take(max_length)
        .collect::<String>();
    (offset, max_length, matched)
}

fn lz77_compress(input: &str, search_buf_len: usize, lookahead_buf_len: usize) -> Vec<Token> {
    let mut encoded = Vec::new();
    let mut search_buffer = String::new();
    let mut pos = 0;
    let end = input.len();

    let input_chars: Vec<char> = input.chars().collect();

    while pos < end {
        let lookahead_end = (pos + lookahead_buf_len).min(end);
        let lookahead: String = input_chars[pos..lookahead_end].iter().collect();

        let (offset, length, matched) = get_match(&search_buffer, &lookahead);

        pos += length;

        let next_char = if pos < input_chars.len() {
            input_chars[pos]
        } else {
            '\0'
        };

        // Add to the search buffer
        search_buffer.push_str(&matched);
        if next_char != '\0' {
            search_buffer.push(next_char);
        }

        // Maintain search buffer length
        if search_buffer.len() > search_buf_len {
            let excess = search_buffer.len() - search_buf_len;
            search_buffer = search_buffer.chars().skip(excess).collect();
        }

        encoded.push(Token {
            offset,
            length,
            next_char,
        });

        pos += 1;
    }

    encoded
}

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
