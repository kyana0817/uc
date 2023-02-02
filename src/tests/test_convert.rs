#[cfg(test)]
mod tests {
    use crate::converts::url::{encode, decode};

    #[test]
    fn test_url_encode() {
        assert_eq!(encode(&"foo <bar>"), "foo%20%3Cbar%3E");
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(decode(&"foo%20%3Cbar%3E"), "foo <bar>");
    }
}