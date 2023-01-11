fn urlify(url: &'static str) -> String {
    url.trim_end().replace(" ", "%20")
}

fn main() {
    println!("Aum Namah Sivaya!!!  {}", urlify("Mr Siva Krishna"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify("Mr Siva Krishna"), "Mr%20Siva%20Krishna");
    }
}
