fn main() {
    let mut words: Vec<String> = std::env::args().skip(1).collect();

    insertion_sort(&mut words);

    for word in words {
        println!("{}", word);
    }
}

fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_sort() {
        let cases = vec![
            vec![],
            vec!["a".to_string()],
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![
                "e".to_string(),
                "d".to_string(),
                "c".to_string(),
                "b".to_string(),
                "a".to_string(),
            ],
            vec![
                "A".to_string(),
                "a".to_string(),
                "A".to_string(),
                "a".to_string(),
                "A".to_string(),
                "a".to_string(),
            ],
        ];

        for mut left in cases {
            let mut right = left.clone();
            insertion_sort(&mut left);

            right.sort();
            assert_eq!(left, right);
        }
    }
}
