use day3::Day3Input;

fn main() {
    let input = day3::get_input();

    let res = run_into_trees(1, 1, (0, 0), &input);

    dbg!(res);
}

fn run_into_trees(right: u32, down: u32, curr_index: (u32, u32), input: &Day3Input) -> u32 {
    let mut counter = 0 as u32;
    let index = (curr_index.0 + down) as usize;
    if index > input.len() {
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

        counter += run_into_trees(right, down, (index as u32, (curr_index.1 + right) % line.len() as u32), &input);
    }

    counter
}
