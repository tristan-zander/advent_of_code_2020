use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct InputFields {
    pub letter: char,
    pub occurances: [i32; 2],
    pub password: String,
}

impl Default for InputFields {
    fn default() -> Self {
        InputFields {
            letter: ' ',
            occurances: [-1, -1],
            password: "".to_owned(),
        }
    }
}

pub fn get_inputs(file_path: &str) -> Vec<InputFields> {
    let mut ret = Vec::<InputFields>::new();
    let mut buf = Vec::new();

    let mut f = File::open(file_path).expect("Could not open given file");
    // This instead of read_to_string so that I know we've read the full file.
    f.read_to_end(&mut buf)
        .expect("Couldn't read file into a buffer");
    let input = String::from_utf8(buf).expect("Could not read buffer into string.");

    for line in input.split('\n') {
        // Read until the first '-'
        // Convert that to occurances[0]
        // Read until next ' '
        // Convert that to occurances[1]
        // Read the next character
        // Convert that to letter
        // Strip unwanted characters and whitespace
        // Convert that to password
        // Add to ret vector.

        let mut inp = InputFields::default();

        let (first_num, rest) = line.split_at(line.find('-').unwrap());
        inp.occurances[0] = first_num.parse().unwrap();

        let (second_num, rest) = rest[1..rest.len()].split_at(rest.find(' ').unwrap());
        inp.occurances[1] = second_num
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let mut rest = rest.split_whitespace();
        inp.letter = rest.next().unwrap().chars().into_iter().next().unwrap();

        inp.password = rest.next().unwrap().to_owned();

        ret.push(inp);
    }

    ret
}
