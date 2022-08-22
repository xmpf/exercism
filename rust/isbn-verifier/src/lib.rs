use std::iter::zip;

// The ISBN-10 format is 9 digits (0 to 9) plus one check character (either a digit or an X only).
// In the case the check character is an X, this represents the value '10'.
// These may be communicated with or without hyphens

pub fn is_valid_digits(isbn: &String) -> bool {
    // contains only 0-9 and X
    isbn.chars()
        .map(|x| if x.is_ascii_digit() || x == 'X' { 0 } else { 1 })
        .sum::<i64>() == 0
}

pub fn is_correct_length(isbn: &String) -> bool {
    // ISBN-10 is 10 char long
    isbn.len() == 10
}

pub fn is_correct_check_char(isbn: &String) -> bool {
    // check if the mark is the last character
    match isbn.find('X') { 
        Some(ix) => ix == 9,
        _ => true
    }
}

pub fn compute_isbn_sum(isbn: &String) -> i64 {
    // compute the formula result
    zip(
        isbn.chars().map(|x| if x == 'X' { 10 } else if let Some(_x) = x.to_digit(10) { _x as i64 } else { 0i64 }),
        vec![10,9,8,7,6,5,4,3,2,1]
    ).fold(0i64, |sum, (x, y)| sum + x * y )
}

pub fn filter_hyphens(isbn: &String) -> String {
    // remove hyphens
    isbn.chars().filter(|&x| x != '-').collect()
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = filter_hyphens(&isbn.chars().collect());

    is_correct_check_char(&isbn)
    && is_correct_length(&isbn)    
    && is_valid_digits(&isbn)
    && compute_isbn_sum(&isbn) % 11 == 0

}
