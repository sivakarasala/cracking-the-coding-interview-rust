use std::collections::HashMap;

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut characters: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if characters.contains_key(&c) {
            if let Some(x) = characters.get_mut(&c) {
                *x += 1;
            }
        } else {
            characters.insert(c, 1);
        }
    }
    characters
}

fn is_permutation(s1: &str, s2: &str) -> bool {
    let s1_chars = count_chars(s1);
    let s2_chars = count_chars(s2);

    for key in s1_chars.keys() {
        if !s2_chars.contains_key(&key) {
            return false;
        }
        if s1_chars.get(&key) != s2_chars.get(&key) {
            return false;
        }
    }
    for key in s2_chars.keys() {
        if !s1_chars.contains_key(&key) {
            return false;
        }
    }
    true
}

fn main() {
    println!("Aum Namah Sivaya!!!  {}", is_permutation("siva", "vasi"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_string_permutation() {
        assert_eq!(is_permutation("levis", "sivel"), true);
        assert_eq!(is_permutation("dog", "cat"), false);
    }
}
