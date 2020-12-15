use std::io::{stdin, Read};

pub fn get_input() -> Vec<u32> {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let res: Vec<_> = buf
        .split("\n\n")
        .map(|group| { 
            let mut characters: Vec<_> = group.chars().filter(|c| *c != '\n').collect();
            characters.sort();
            characters.dedup();

            // Since duplicates are already removed
            characters.iter().fold(0, |acc, _| {
                acc + 1
            })
        })
        .collect();

    res
}
