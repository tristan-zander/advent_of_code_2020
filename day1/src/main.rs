use std::fs::File;
use std::{collections::HashMap, io::Read};

fn main() {
    let mut buf = String::new();
    let mut input = Vec::<i32>::new();
    let mut file = File::open("input").unwrap();
    let _lines_read = file.read_to_string(&mut buf).unwrap();
    let vectored_file = buf.split("\n");
    for line in vectored_file {
        if line.len() > 0 {
            input.push(line.parse().unwrap());
        }
    }

    let (mut answer1, mut answer2) = (0, 0);

    // Key: opposite, Value: number
    let mut hash = HashMap::<i32, i32>::new();
    for &num in input.iter() {
        let opposite = 2020 - num;
        if let Some(_) = hash.get(&num) {
            answer1 = num;
            answer2 = opposite;
            break;
        }
        hash.insert(opposite, num);
    }

    if answer1 > 0 && answer2 > 0 {
        println!("{}", answer1 * answer2);
    } else {
        panic!("Failed to find a suitable pair of numbers.");
    }
}
