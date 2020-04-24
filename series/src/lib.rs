pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }

    (0..digits.len() - len + 1)
        .map(|i| digits[i..(i + len)].to_string())
        .collect()
}
