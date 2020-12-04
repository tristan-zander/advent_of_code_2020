use day2::*;

fn main() {
    let inputs = get_inputs("../input");
    let mut valid_passwords = 0;
    for inp in inputs.iter() {
        let index1 = inp.password.as_bytes()[inp.occurances[0] as usize - 1];
        let index2 = inp.password.as_bytes()[inp.occurances[1] as usize - 1];        
        if (index1 == inp.letter as u8 || index2 == inp.letter as u8) && index1 != index2 {
            valid_passwords += 1;
        }
    }
    dbg!(valid_passwords);
}
