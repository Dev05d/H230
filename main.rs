fn store_message(content: String) -> usize {
    println!("Storing message: \"{}\"", content);
    content.len()
}

fn validate_capacity(limit: i32) -> bool {
    println!("Validating capacity: {}", limit);
    limit > 0 && limit <= 100
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_string_ownership_moves() {
        let msg = String::from("hello from room");
        let len = store_message(msg); 
        assert_eq!(len, 15);
    }

    #[test]
    fn test_i32_is_copied() {
        let capacity = 42;
        let is_valid = validate_capacity(capacity);
        assert!(is_valid);

        println!("capacity {} is still usable after the call", capacity);
        assert_eq!(capacity, 42);
    }
}