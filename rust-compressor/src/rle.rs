pub fn compress_rle(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 1;
        while i + 1 < chars.len() && chars[i] == chars[i + 1] {
            count += 1;
            i += 1;
        }
        result.push(chars[i]);
        result.push_str(&count.to_string());
        i += 1;
    }

    result
}

pub fn decompress_rle(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count_str = String::new();
        while let Some(&next_char) = chars.peek() {
            if next_char.is_ascii_digit() {
                count_str.push(next_char);
                chars.next();
            } else {
                break;
            }
        }
        let count = count_str.parse::<usize>().unwrap_or(1);
        result.push_str(&current_char.to_string().repeat(count));
    }

    result
}
