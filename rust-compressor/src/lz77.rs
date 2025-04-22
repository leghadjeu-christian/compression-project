// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

// use std::mem;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct LZ77Token {
//     length: usize,
//     distance: usize,
//     char: u8,
// }

// pub struct LZ77Algorithm {}

// impl LZ77Algorithm {
//     pub fn get_distance_to_match(data: &[u8], offset: usize, end: usize) -> u8 {
//         let mut offset: usize = offset;
//         let mut end: usize = end;
//         let mut len: u8 = 0;

//         // if item are equal, we increment the offset and the position
//         while offset < end && end < data.len() && data[offset] == data[end] && len < 255 {
//             offset += 1;
//             end += 1;
//             len += 1;
//         }

//         return len;
//     }

//     pub fn find_match(data: &[u8], end: usize) -> (u8, u8) {
//         let mut best_offset = 0u8;
//         let mut best_len = 0u8;

//         for offset in 0..end {
//             // end in this case is the end of the data stream
//             let len = LZ77Algorithm::get_distance_to_match(data, offset, end);
//             if len > best_len {
//                 best_offset = (end - offset) as u8; // distance from index (end)
//                 best_len = len; // length of the match
//             }
//         }
//         return (best_offset, best_len);
//     }

//     pub fn encode(&self, input: &str) -> Vec<LZ77Token> {
//         let input_as_byte = input.as_bytes();
//         let mut output: Vec<LZ77Token> = Vec::new();
//         let mut buffer: Vec<u8> = Vec::new();
//         let mut index: usize = 0; // do not use a for since it will iterate over the entire set of
//                                   // elements

//         while index < input_as_byte.len() {
//             let (offset, len) = LZ77Algorithm::find_match(&input_as_byte, index);

//             let mut new_token = LZ77Token {
//                 distance: offset as usize,
//                 length: 0,
//                 char: 0,
//             };

//             if offset == 0 {
//                 new_token.char = input_as_byte[index];
//                 output.push(new_token);
//                 index += 1;
//             } else {
//                 new_token.length = len as usize;
//                 output.push(new_token);
//                 index += len as usize;
//             }
//         }

//         output
//     }

//     pub fn decode(&self, data: &Vec<LZ77Token>) -> String {
//         let mut output = String::new();

//         for (index, token) in data.iter().enumerate() {
//             if token.length == 0 {
//                 output.push(token.char as char);
//             } else {
//                 let start = output.len() - token.distance;
//                 let end = start + token.length;

//                 for i in start..end {
//                     let c = output.chars().nth(i).unwrap();
//                     output.push(c);
//                 }
//             }
//         }

//         return output;
//     }

//     pub fn new() -> LZ77Algorithm {
//         LZ77Algorithm {}
//     }
// }



// // add test
// #[cfg(test)]
// mod test_suite {

//     use super::LZ77Algorithm;

//     #[test]
//     fn encode_length_three_string() {
//         let lz77 = LZ77Algorithm::new();

//         // input to compress
//         let input = "abc";

//         // encode input
//         let encoded_result = lz77.encode(input);
//         let decoded_result = lz77.decode(&encoded_result);

//         assert_eq!(decoded_result, input);
//     }

//     #[test]
//     fn encode_length_complex_string() {
//         let lz77 = LZ77Algorithm::new();

//         // input to compress
//         let input = "aabbcc";

//         // encode input
//         let encoded_result = lz77.encode(input);
//         let decoded_result = lz77.decode(&encoded_result);

//         assert_eq!(decoded_result, input);
//     }
// }

#[derive(Debug)]
struct LZ77Token {
    offset: usize,
    length: usize,
    next_char: char,
}

fn lz77_encode_with_struct(data: &str, search_buffer_size: usize, lookahead_buffer_size: usize) -> Vec<LZ77Token> {
    let mut encoded_data = Vec::new();
    let data_bytes = data.as_bytes();
    let data_len = data_bytes.len();
    let mut current_position = 0;

    while current_position < data_len {
        let search_buffer_start = usize::max(0, (current_position as isize - search_buffer_size as isize).try_into().unwrap()) as usize;
        let search_buffer = &data_bytes[search_buffer_start..current_position];

        let lookahead_end = usize::min(current_position + lookahead_buffer_size, data_len);
        let lookahead_buffer = &data_bytes[current_position..lookahead_end];

        let mut best_match_offset = 0;
        let mut best_match_length = 0;

        if !lookahead_buffer.is_empty() {
            for offset in 1..=search_buffer.len() {
                for length in 1..=lookahead_buffer.len() {
                    if offset > search_buffer.len() {
                        continue;
                    }
                    let search_start = search_buffer.len() - offset;
                    let matched_segment = &search_buffer[search_start..];

                    if length <= matched_segment.len() && lookahead_buffer[..length] == matched_segment[..length] {
                        if length > best_match_length {
                            best_match_length = length;
                            best_match_offset = offset;
                        }
                    } else if length > matched_segment.len() && lookahead_buffer[..matched_segment.len()] == *matched_segment {
                        if matched_segment.len() > best_match_length {
                            best_match_length = matched_segment.len();
                            best_match_offset = offset;
                        }
                    }
                }
            }
        }

        if best_match_length > 0 {
            let next_char = if current_position + best_match_length < data_len {
                data_bytes[current_position + best_match_length] as char
            } else {
                '\0' // Or some other indicator if at the end
            };
            encoded_data.push(LZ77Token {
                offset: best_match_offset,
                length: best_match_length,
                next_char,
            });
            current_position += best_match_length + 1;
        } else if current_position < data_len {
            encoded_data.push(LZ77Token {
                offset: 0,
                length: 0,
                next_char: data_bytes[current_position] as char,
            });
            current_position += 1;
        } else {
            break;
        }
    }

    encoded_data
}

fn lz77_decode_with_struct(encoded_data: &[LZ77Token]) -> String {
    let mut decoded_data = String::new();
    for token in encoded_data {
        if token.offset == 0 && token.length == 0 {
            decoded_data.push(token.next_char);
        } else {
            if token.offset <= decoded_data.len() {
                let start = decoded_data.len() - token.offset;
                for i in 0..token.length {
                    if start + i < decoded_data.len() {
                        decoded_data.push(decoded_data.as_bytes()[start + i] as char);
                    }
                }
            }
            if token.next_char != '\0' {
                decoded_data.push(token.next_char);
            }
        }
    }
    decoded_data
}

fn main() {
    let input_data = "abcbbcbaaaaaa";
    let search_buffer_size = 6;
    let lookahead_buffer_size = 5;

    println!("Original data: {}", input_data);

    let encoded = lz77_encode_with_struct(input_data, search_buffer_size, lookahead_buffer_size);
    println!("Encoded data (with struct): {:?}", encoded);

    let decoded = lz77_decode_with_struct(&encoded);
    println!("Decoded data: {}", decoded);

    assert_eq!(input_data, decoded);
    println!("Decoding successful!");
}
