pub fn abbreviate(phrase: &str) -> String {
    let mut space_or_hyphen: bool = true;
    let mut previous_char_is_lowercase: bool = false;
    let mut ret: String = String::new();
    for c in phrase.chars() {
        if c.is_uppercase() && space_or_hyphen {
            space_or_hyphen = false;
            ret.push(c);
        } else if c == ' ' || c == '-' {
            space_or_hyphen = true;
            previous_char_is_lowercase = false;
        } else if c.is_lowercase() {
            previous_char_is_lowercase = true;
            if space_or_hyphen {
                space_or_hyphen = false;
                ret.push(c.to_ascii_uppercase());
            }
        } else if c.is_uppercase() && previous_char_is_lowercase {
            previous_char_is_lowercase = false;
            ret.push(c);
        }
    }
    ret
}
