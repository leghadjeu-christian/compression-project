use crate::custom::Code;
use num_traits::Bounded;
use std::convert::{TryFrom, TryInto};

/// Convenience method that applies LZ77 to a string.
///
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn from_string<U>(input: &str) -> Vec<Code<char, U>>
where
    U: TryInto<usize> + TryFrom<usize> + Bounded + Copy,
{
    let input_chars: Vec<char> = input.chars().collect();
    return compress_lz::<char, U>(&input_chars);
}

/// The LZ77 compression, the turning input into a sequence of Codes.
///
/// T: the type that the input consists of.
///
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn compress_lz<T, U>(input: &[T]) -> Vec<Code<T, U>>
where
    T: Eq + Copy,
    U: TryInto<usize> + TryFrom<usize> + Bounded + Copy,
{
    let mut encoded: Vec<Code<T, U>> = Vec::with_capacity(input.len());
    let mut position: usize = 0;
    while position < input.len() {
        let lookahead = &input[position..];
        let window_start = position.saturating_sub(U::max_value().try_into().ok().unwrap());
        let window = &input[window_start..position];
        let code: Code<T, U> = find_code(&window, &lookahead);
        position += code.length.try_into().ok().unwrap() + 1;
        encoded.push(code);
    }
    return encoded;
}

/// Computes the next Code, given the window and a non-empty lookahead.
///
/// T: the type that the input consists of.
///
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
fn find_code<T, U>(window: &[T], lookahead: &[T]) -> Code<T, U>
where
    T: Eq + Copy,
    U: TryFrom<usize>,
{
    let mut lookahead_iterator = lookahead.iter();
    let mut code = Code::<T, usize> {
        offset: 0,
        length: 0,
        literal: *lookahead_iterator.next().unwrap(),
    };
    let mut search_length: usize = 1;
    while search_length < lookahead.len() {
        let search = &lookahead[..search_length];
        let rightmost_match = rfind(window, search);
        if rightmost_match == None {
            break;
        }
        code.offset = window.len() - rightmost_match.unwrap();
        code.length = search_length.into();
        code.literal = *lookahead_iterator.next().unwrap();
        search_length += 1;
    }
    return Code::<T, U> {
        offset: U::try_from(code.offset).ok().unwrap(),
        length: U::try_from(code.length).ok().unwrap(),
        literal: code.literal,
    };
}

/// Returns the position of the rightmost match of search in the given window, if any.
///
/// T: the type that the input consists of.
fn rfind<T>(window: &[T], search: &[T]) -> Option<usize>
where
    T: Eq,
{
    if search.len() > window.len() {
        return None;
    }
    let mut position: usize = window.len() - search.len();
    'scan: loop {
        let segment = &window[position..(position + search.len())];
        for (segment_next, search_next) in segment.iter().zip(search.iter()) {
            if segment_next != search_next {
                if position == 0 {
                    break 'scan;
                }
                position -= 1;
                continue 'scan;
            }
        }
        return Some::<usize>(position);
    }
    return None;
}

/// Convenience method that reconstructs a string from a sequence of Code<char, U>.
///
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn to_string<U>(encoded: &[Code<char, U>]) -> String
where
    U: TryInto<usize> + Copy,
{
    let decoded = decode::<char, U>(encoded);
    return decoded.into_iter().collect();
}

/// Reconstructs the original data from a sequence of Codes.
///
/// T: the type that the input consists of.
///
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn decode<T, U>(encoded: &[Code<T, U>]) -> Vec<T>
where
    T: Copy,
    U: TryInto<usize> + Copy,
{
    let mut capacity: usize = 0;
    for code in encoded.iter() {
        capacity += code.length.try_into().ok().unwrap() + 1;
    }
    let mut decoded = Vec::<T>::with_capacity(capacity);
    let mut position: usize = 0;
    for code in encoded.iter() {
        if code.length.try_into().ok().unwrap() > 0 {
            let segment_start = position - code.offset.try_into().ok().unwrap();
            let segment_end = segment_start + code.length.try_into().ok().unwrap();
            let segment = &mut decoded[segment_start..segment_end].to_owned();
            decoded.append(segment);
            position += code.length.try_into().ok().unwrap();
        }
        decoded.push(code.literal);
        position += 1;
    }
    return decoded;
}
