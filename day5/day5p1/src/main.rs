use std::io::BufRead;
fn main() {
    let max = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.chars().fold(0, |acc, el| {
                (acc << 1) | if el == 'B' || el == 'R' {1} else {0}
            })
        }).max().unwrap();
    dbg!(max);
}
