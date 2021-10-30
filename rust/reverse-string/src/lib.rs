pub fn reverse(input: &str) -> String {
    // `chars()` => returns an iterator over the characters of a string slice
    // `rev()` => reverses iterator's direction
    // `collect()` => transforms an iterator into a collection
    input.chars().rev().collect::<String>()
}
