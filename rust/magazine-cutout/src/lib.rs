// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    
    let mut map_a: HashMap<&str, i32> = HashMap::new();
    
    // build frequency table
    for word_a in magazine {
        // entry returns a reference
        *map_a.entry(word_a).or_insert(0) += 1;
    }

    // decrement for each word in note
    for word_b in note {
        // entry returns a reference
        *map_a.entry(word_b).or_insert(-1) -= 1;
        match map_a.get(word_b) {
            Some(&n) if n >= 0  => continue,
            _                   => return false,
        }
    }
    return true;
}
