use std::io::Read;

fn main() {
    let handle: std::io::Stdin = std::io::stdin();
    println!("{}", get_bytes_count(handle));
}

fn get_bytes_count<T>(mut src: T) -> usize
where
    T: Read,
{
    let mut temp = Vec::new();
    let _ = src.read_to_end(&mut temp);

    let bytes: usize = temp.len();

    return bytes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multy_cases() {
        let big = vec![0u8; 1 << 15];
        let cases = vec![
            (&b""[..], 0),
            (&b"abcd"[..], 4),
            (&b"abcd\r"[..], 5),
            (&b"abcd/r"[..], 6),
            (&b"abcd/r/n"[..], 8),
            (&big, 1 << 15),
            ("🦀".as_bytes(), 4),
        ];
        for (input, output) in cases {
            assert_eq!(get_bytes_count(input), output)
        }
    }
}
