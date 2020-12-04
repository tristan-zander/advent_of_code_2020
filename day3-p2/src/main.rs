use day3::Day3Input;

fn main() {
    let input = day3::get_input();

    let d1r1 = run_into_trees(1, 1, (0, 0), &input);
    let d1r3 = run_into_trees(3, 1, (0, 0), &input);
    let d1r5 = run_into_trees(5, 1, (0, 0), &input);
    let d1r7 = run_into_trees(7, 1, (0, 0), &input);
    let d2r1 = run_into_trees(1, 2, (0, 0), &input);

    dbg!(d1r5 * d1r3 * d2r1 * d1r7 * d1r1);
}

fn run_into_trees(right: u32, down: u32, curr_index: (u32, u32), input: &Day3Input) -> u32 {
    let mut counter = 0 as u32;
    let index = (curr_index.0 + down) as usize;
    if index >= input.len() {
        return counter;
    }
    let line = &input.data[index];
    if line.len() == 0 {
        return counter;
    }
    if let Some(tree) = line
        .chars()
        .nth(((curr_index.1 + right) % line.len() as u32) as usize)
    {
        if tree == '#' {
            counter += 1;
        }

        counter += run_into_trees(
            right,
            down,
            (index as u32, (curr_index.1 + right) % line.len() as u32),
            &input,
        );
    }

    counter
}
