fn main() {
    let input = day3::get_input();
    let mut inp_iter = input.iter();

    inp_iter.next(); // Skip the first line

    let mut counter: u32 = 0;
    for (i, line) in input.iter().enumerate() {
        if line.len() == 0 {
            // Last line is blank for some reason.
            break;
        }
        if let Some(tree) = line.chars().nth((i * 3) % line.len()) {
            if tree == '#' {
                counter += 1;
            }
        }
    }

    println!("Ran into {} trees.", counter);
}
