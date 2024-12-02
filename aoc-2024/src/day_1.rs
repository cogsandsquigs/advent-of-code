use utils::{itertools::Itertools, macros::solution};

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|x| x.split_whitespace())
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unzip()
}

#[solution(day = 1, part = 1)]
fn part_1(input: &str) -> usize {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

#[solution(day = 1, part = 2)]
fn part_2(input: &str) -> usize {
    let (left, right) = parse(input);

    let right_set = right.iter().counts_by(|x| x);

    left.iter()
        .map(|x| x * right_set.get(x).unwrap_or(&0))
        .sum()
}
