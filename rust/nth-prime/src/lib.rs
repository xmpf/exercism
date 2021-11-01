pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).take((n + 1) as usize).last().unwrap()
}

pub fn is_prime(n: u32) -> bool {
    !(2..(n/2+1)).any(|x| n % x == 0)
}