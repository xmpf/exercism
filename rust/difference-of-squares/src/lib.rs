pub fn square_of_sum(n: u32) -> u32 {
    let sum: u32 = (n * (n + 1)) >> 1;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sq_sum = 1;
    for i in 2..=n {
        sq_sum += i * i;
    }
    sq_sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
