pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] { 
    let mut result: [u8; 8] = [0; 8];
    for (i, line) in lines.iter().enumerate() {
        let mut x: u8 = 0b0000_0000;
        let len = line.len();
  
        for (j, y) in line.bytes().enumerate() {
            let n = len - j - 1;
            if y == b'#' {
                x = x | (1 << n);
            }
        }
        result[i] = x;
    }
    return result;
} 
pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] { 
    let result = bytes.map(render_byte_to_string);
    return result;
} 

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] { 
    let result = bytes.map(|x| !x);
    return result;
}

fn render_byte_to_string(byte: u8) -> String {
    let mut result: String = String::new();
    let mut i: i32 = 7;

    while i >= 0 {
        if byte & (1 << i) != 0 {
            result.push('#');
        } else {
            result.push('.');
        }
        i-=1;
    }
    
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bitmap_8x8_test() {
        let image = [
            "..####..", 
            ".#....#.", 
            "#.#..#.#", 
            "#..##..#", 
            "#......#", 
            "#.#..#.#", 
            ".#....#.", 
            "..####..", 
        ];

        let bytes = parse_bitmap_8x8(image);

        let expected = [
            0b0011_1100, 
            0b0100_0010, 
            0b1010_0101, 
            0b1001_1001, 
            0b1000_0001, 
            0b1010_0101, 
            0b0100_0010, 
            0b0011_1100,
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn render_bitmap_8x8_test() {
        let bytes = [
            0b0011_1100, 
            0b0100_0010, 
            0b1010_0101, 
            0b1001_1001, 
            0b1000_0001, 
            0b1010_0101, 
            0b0100_0010, 
            0b0011_1100, 
        ];

        let image = render_bitmap_8x8(bytes);
        let expected = [
            "..####..",
            ".#....#.", 
            "#.#..#.#", 
            "#..##..#", 
            "#......#", 
            "#.#..#.#", 
            ".#....#.", 
            "..####..", 
        ];

        assert_eq!(image, expected.map(|x: &str| x.to_string()));
    }

    #[test]
    fn invert_bitmap_8x8_test() {
        let source = [
            "..####..", 
            ".#....#.", 
            "#.#..#.#", 
            "#..##..#", 
            "#......#", 
            "#.#..#.#", 
            ".#....#.", 
            "..####..",
        ];

        let bytes = parse_bitmap_8x8(source);
        let inverted_bytes = invert_bitmap_8x8(bytes);
        let image = render_bitmap_8x8(inverted_bytes);

        let expected = [
            "##....##", 
            "#.####.#", 
            ".#.##.#.", 
            ".##..##.", 
            ".######.", 
            ".#.##.#.", 
            "#.####.#", 
            "##....##",
        ];
        assert_eq!(image, expected);
    }
}