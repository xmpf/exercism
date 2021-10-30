pub fn first_ln(n: u32) -> String {
    // 99 bottles of beer on the wall, 99 bottles of beer.
    // 1 bottle of beer on the wall, 1 bottle of beer.
    // No more bottles of beer on the wall, no more bottles of beer.
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer."),      
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer."),
        n => format!("{} bottles of beer on the wall, {} bottles of beer.", n, n),
    }
}

pub fn second_ln(n: u32) -> String {
    // Take one down and pass it around, 98 bottles of beer on the wall.
    // Take one down and pass it around, 1 bottle of beer on the wall.
    // Go to the store and buy some more, 99 bottles of beer on the wall.
    match n {
        0 => String::from("Go to the store and buy some more, 99 bottles of beer on the wall."),      
        1 => String::from("Take it down and pass it around, no more bottles of beer on the wall."),
        2 => String::from("Take one down and pass it around, 1 bottle of beer on the wall."),
        n => format!("Take one down and pass it around, {} bottles of beer on the wall.", n - 1),
    }
}

pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", first_ln(n), second_ln(n))
}

pub fn sing(start: u32, end: u32) -> String {
    let song: String =
        (end..=start)
        .rev()
        .map(verse)
        .fold(String::new(), |acc, part| acc + &part + "\n");
    song[..song.len() - 1].to_string()
}
