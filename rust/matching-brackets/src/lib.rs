pub fn brackets_are_balanced(string: &str) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in string.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    match stack.pop() {
                        Some(x) if x != c => return false,
                        None => return false,
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        stack.is_empty()
}
