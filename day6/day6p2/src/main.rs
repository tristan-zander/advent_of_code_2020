use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    let _ = stdin().lock().read_to_string(&mut buf);

    let split: Vec<_> = buf
        .split("\n\n")
        .map(|grp| {
            let line_count = grp.lines().count();
            let mut chars: Vec<_> = grp.chars().filter(|v| *v != '\n').collect();
            chars.sort();

            let mut curr_char = ' ';
            let mut occurances = 0;
            let mut res: u32 = 0;
            for &l in chars.iter() {
                if l == curr_char {
                    occurances += 1;
                } else {
                    curr_char = l;
                    occurances = 1;
                }
                if occurances >= line_count {
                    res += 1;
                }
            }

            dbg!(chars, line_count, res);
            res
        })
        .collect();

    println!("{:?}", split.iter().sum::<u32>());
}
