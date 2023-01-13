fn compress_string(s: &str) -> String {
    let mut count = 0;
    let mut result = s.chars().fold(String::new(), |mut result, current_char| {
        if result.is_empty() {
            result.push(current_char);
            count = 1;
        } else {
            if current_char == result.chars().last().unwrap() {
                count += 1;
            } else {
                result.push_str(&format!("{}", count));
                count = 1;
                result.push(current_char);
            }
        }
        result
    });
    result.push_str(&format!("{}", count));

    if s.len() < result.len() {
        return String::from(s);
    }
    result
}

fn main() {
    println!("Aum Namah Sivaya!!!, {}", compress_string("aabcccccaaa"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_compression() {
        assert_eq!(compress_string("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(
            compress_string("woidfhgladhlkejofiajmldkjhfaoj"),
            "woidfhgladhlkejofiajmldkjhfaoj"
        );
    }
}
