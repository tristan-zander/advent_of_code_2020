use day4::get_input;

fn main() {
    let input = get_input();

    let mut counter = 0 as u64;
    for passport in input {
        if let None = passport.hgt {continue;}
        if passport.byr != None
            && passport.iyr != None
            && passport.eyr !=  None
            && passport.hcl != None
            && passport.ecl != None
            && passport.pid != None
        { counter += 1; }
    }

    dbg!(counter);
}
