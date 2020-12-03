use std::{fs::File, io::Read};

fn main() {
    let mut buf = String::new();
    let mut input = Vec::<i32>::new();
    let mut file = File::open("../input").unwrap();
    file.read_to_string(&mut buf).unwrap();
    let vectored_file = buf.split("\n");
    for line in vectored_file {
        if line.len() > 0 {
            input.push(line.parse().unwrap());
        }
    }

    let (mut ans1, mut ans2, mut ans3) = (0,0,0); 

    for &i in input.iter() {
        for &j in input.iter() {
            for &k in input.iter() {
                if i + j + k == 2020 {
                    ans1 = i;
                    ans2 = j;
                    ans3 = k;
                    break;
                }
            }
        }
    }

    println!("{}", ans1 * ans2 * ans3);
}
