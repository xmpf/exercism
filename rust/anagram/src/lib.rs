use std::collections::HashSet;

// Solution from: https://exercism.org/tracks/rust/exercises/anagram/solutions/Nvveen

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut sorted_word = word.chars().collect::<Vec<char>>();
    sorted_word.sort();

    possible_anagrams
        .iter()
        .filter(|&w| {
            // convert to lowercase
            let w = w.to_lowercase();

            // sort by value
            let mut w_sorted = w.chars().collect::<Vec<char>>();
            w_sorted.sort();

            // anagrams have to contain the same values in the vector
            // and need not to be the same with the given word.
            return w != word
            && w_sorted == sorted_word
        })
        .cloned()   // &&T => &T
        .collect()
}