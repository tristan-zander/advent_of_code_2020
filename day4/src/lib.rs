/// Options indicate invalid passport entries.
#[derive(Default, Debug, Copy, Clone)]
pub struct Day4Input<'a> {
    pub byr: Option<u32>,
    pub iyr: Option<u32>,
    pub eyr: Option<u32>,
    pub hgt: Option<(HeightType, u32)>,
    pub hcl: Option<(u8, u8, u8)>,
    pub ecl: Option<&'a str>,
    pub pid: Option<u32>,
    pub cid: Option<u32>,
}

#[derive(Debug, Copy, Clone)]
pub enum HeightType {
    IMPERIAL,
    METRIC,
}

pub fn get_input<'a>() -> Vec<Day4Input<'a>> {
    let mut res = Vec::new();
    let f: Vec<&str> = include_str!("../input").split("\n\n").collect();

    for passport_entry in f {
        let mut passport = Day4Input::default();

        let entries: Vec<&str> = passport_entry.split('\n').collect();
        for line in entries {
            let full_entry: Vec<&str> = line.split_whitespace().collect();
            for pair in full_entry {
                let token = pair.split(':').nth(0).unwrap().trim();
                let value = pair.split(':').nth(1).unwrap().trim();

                match token {
                    "byr" => {
                        let value = value.parse().unwrap();
                        if value >= 1920 && value <= 2002 {
                            passport.byr = Some(value);
                        }
                    }
                    "iyr" => {
                        let value = value.parse().unwrap();
                        if value >= 2010 && value <= 2020 {
                            passport.iyr = Some(value);
                        }
                    }
                    "eyr" => {
                        let value = value.parse().unwrap();
                        if value >= 2020 && value <= 2030 {
                            passport.eyr = Some(value);
                        }
                    }
                    "hgt" => {
                        if let Some(inches) = value.find("in") {
                            let value: u32 = (&value[0..inches]).parse().unwrap();
                            passport.hgt = Some((HeightType::IMPERIAL, value));
                        } else if let Some(cm) = value.find("cm") {
                            let value: u32 = (&value[0..cm]).parse().unwrap();
                            passport.hgt = Some((HeightType::METRIC, value));
                        }
                    }
                    "hcl" => {
                        // If it's not hex, it's invalid
                        if value.find('#') == None {
                            continue;
                        } else {
                            let value = &value[1..].trim();
                            let value = hex::decode(&value.as_bytes());
                            match value {
                                Ok(x) =>  passport.hcl = Some((x[0], x[1], x[2])),
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                    }
                    "ecl" => match value {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                            passport.ecl = Some(value)
                        }
                        _ => continue,
                    },
                    "pid" => {
                        if value.trim().len() == 9 {
                            match value.trim().parse() {
                                Ok(x) => passport.pid = Some(x),
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                    }
                    "cid" => {
                        passport.cid = Some(value.parse().unwrap());
                    }
                    _ => {
                        eprintln!("Invalid token: {}", token);
                    }
                }
            }
        }
        res.push(passport);
    }
    res
}
