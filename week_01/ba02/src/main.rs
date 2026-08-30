use std::io::Read;

fn main() {
    let handle: std::io::Stdin = std::io::stdin();
    let (lines, words, bytes) = get_bytes_count(handle);
    println!("{} {} {}", lines, words, bytes);
}

fn get_bytes_count<T>(mut src: T) -> (usize, usize, usize)
where
    T: Read,
{
    let mut temp = Vec::new();
    _ = src.read_to_end(&mut temp);

    let text = String::from_utf8_lossy(&temp);

    let lines = text.chars().filter(|&c| c == '\n').count();
    let words: usize = text.split_whitespace().collect::<Vec<&str>>().len();
    let bytes: usize = temp.len();

    return (lines, words, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multy_cases() {
        let big = vec![0u8; 1 << 15];
        let cases = vec![
            (&b""[..], (0, 0, 0)),
            (&b"hello"[..], (0, 1, 5)),
            (&b"hello\n"[..], (1, 1, 6)),
            (&b"hello rust\n"[..], (1, 2, 11)),
            (&b" hello rust \n"[..], (1, 2, 13)),
            (&b"a\tb\nc"[..], (1, 3, 5)),
        ];
        for (input, output) in cases {
            assert_eq!(get_bytes_count(input), output)
        }
    }
}
