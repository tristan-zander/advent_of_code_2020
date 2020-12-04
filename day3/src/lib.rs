use std::ops::Deref;

pub struct Day3Input {
    pub data: Vec<String>,
}

impl Deref for Day3Input {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn get_input() -> Day3Input {
    let f = String::from_utf8(include_bytes!("../input").to_vec()).unwrap();

    let mut f_vec = Vec::new();
    for line in f.split("\n") {
        f_vec.push(line.to_string());
    }

    dbg!(f_vec.clone());

    Day3Input { data: f_vec }
}
