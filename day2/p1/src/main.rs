use day2::get_inputs;
fn main() {
    let inputs = get_inputs("../input");
    let mut valid_passwords = 0;
    for inp in inputs.iter() {
        let mut counter = -1; // To account for the fact that split will add an extra element at the end.
        for _ in inp.password.split(inp.letter) {
            counter += 1;
        }
        if counter >= inp.occurances[0] && counter <= inp.occurances[1] {
            valid_passwords += 1;
            continue;
        }
    }
    dbg!(valid_passwords);
}
