use std::io::BufRead;
fn main() {
    let mut passes: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.chars().fold(0, |acc, el| {
                (acc << 1) | if el == 'B' || el == 'R' { 1 } else { 0 }
            })
        })
        .collect();

    passes.sort();
    let result = &(passes[0]..=passes[passes.len() - 1])
        .zip(passes.iter())
        .find(|(expected, seat_id)| expected != *seat_id)
        .unwrap();

    dbg!(result);
}
