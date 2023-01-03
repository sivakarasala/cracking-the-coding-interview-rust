use std::collections::HashSet;

fn all_chars_unique_part_a(s: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if chars.contains(&c) {
            return false;
        }
        chars.insert(c);
    }
    true
}

fn all_chars_unique_part_b(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let a_int_char: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char = c as i16;
        int_char -= a_int_char;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit
        bitfield |= 1 << int_char;
        // println!("{:b}", bitfield);
    }

    true
}

fn main() {
    println!("{}", all_chars_unique_part_a("helloworld"));
    println!("{}", all_chars_unique_part_b("helloworld"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique_part_a("abcdefg"), true);
        assert_eq!(all_chars_unique_part_a("abcdefga"), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_part_b("aeiou"), true);
        assert_eq!(all_chars_unique_part_b("abcdefga"), false);
    }
}
