fn is_rotated_string(s1: &str, s2: &str) -> bool {
    let triplet = format!("{}{}{}", s1, s1, s1);
    triplet.contains(s2)
}

fn main() {
    println!("Aum Namah Sivaya!!! {}", is_rotated_string("siva", "ivas"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_rotation() {
        assert_eq!(is_rotated_string("waterbottle", "erbottlewat"), true);
        assert_eq!(is_rotated_string("herpderp", "history"), false);
    }
}
