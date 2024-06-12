pub fn is_palindrome(input: &str) -> bool {
    let input = input.trim().to_lowercase();
    let len = input.len();

    for i in 0..len / 2 {
        if input.as_bytes()[i] != input.as_bytes()[len - 1 - i] {
            return false;
        }
    }

    true
}
