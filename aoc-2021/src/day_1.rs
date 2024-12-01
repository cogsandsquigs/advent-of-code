use utils::{itertools::Itertools, macros::solution};

#[solution(day = 1, part = 1)]
fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b)| usize::from(a < b))
        .sum()
}

#[solution(day = 1, part = 2)]
fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec()
        .windows(3)
        .map(|list| list.iter().sum::<usize>())
        .tuple_windows()
        .map(|(a, b)| usize::from(a < b))
        .sum()
}
