use day4::*;

fn main() {
    let input = get_input();

    let mut counter = 0 as u32;
    for &passport in input.iter() {
        if let Some((t, val)) = passport.hgt {
            match t {
                HeightType::METRIC if (val >= 150 && val <= 193) => {}
                HeightType::IMPERIAL if (val >= 59 && val <= 76) => {}
                _ => continue,
            }
        } else {
            continue;
        }
        if passport.byr != None
            && passport.iyr != None
            && passport.eyr != None
            && passport.hcl != None
            && passport.pid != None
            && passport.ecl != None
        {
            counter += 1;
        }
    }
    dbg!(counter);
}
