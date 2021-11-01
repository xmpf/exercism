use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let mut multiples = HashSet::new();
    let mut current;
    for i in 0..factors.len() {
        if factors[i] == 0 {
            continue;
        }
        current = factors[i];
        while current < limit {
            multiples.insert(current);
            current += factors[i];
        }
    }
    multiples.iter().fold(0, |acc, x| acc + x)
}
