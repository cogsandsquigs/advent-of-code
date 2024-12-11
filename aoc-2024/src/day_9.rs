use utils::{itertools::Itertools, macros::solution};

#[solution(day = 8, part = 1)]
fn part_1(input: &str) -> u32 {
    let ns = input.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
    let mut files = vec![];
    let mut file_id = 0;
    let mut is_file = true;
    let mut working_on = (ns.len() - 1, 0);

    for i in 0..ns.len() {
        if i == working_on.0 {
            break;
        }

        if is_file {
            let mut tmp = vec![file_id; ns[i] as usize];
            files.append(&mut tmp);
            file_id += 1;
        } else {
            let mut tmp = vec![working_on.0 / 2; ns[i] as usize];
            files.append(&mut tmp);

            if ns[i] == ns[working_on.0] {}
        }

        is_file = !is_file;
    }

    0
}

#[solution(day = 8, part = 2)]
fn part_2(input: &str) -> usize {
    0
}
