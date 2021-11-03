pub fn is_armstrong_number(num: u32) -> bool {
    let length: u32 = num.to_string().len() as u32;
    num.to_string().chars().fold(0, |acc: u32, x: char| acc + (x.to_digit(10).unwrap()).pow(length)) == num
}
