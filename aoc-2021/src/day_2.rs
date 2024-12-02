use utils::macros::solution;

#[solution(day = 2, part = 1)]
fn part_1(input: &str) -> usize {
    let (horiz, depth) = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut v| {
            (
                v.next().unwrap(),
                v.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold((0, 0), |(horiz, depth), (instr, val)| match instr {
            "forward" => (horiz + val, depth),
            "up" => (horiz, depth - val),
            "down" => (horiz, depth + val),
            _ => unreachable!(),
        });

    horiz * depth
}

#[solution(day = 2, part = 2)]
fn part_2(input: &str) -> usize {
    let (horiz, depth, _) = input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut v| {
            (
                v.next().unwrap(),
                v.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(horiz, depth, aim), (instr, val)| match instr {
            "forward" => (horiz + val, depth + aim * val, aim),
            "up" => (horiz, depth, aim - val),
            "down" => (horiz, depth, aim + val),
            _ => unreachable!(),
        });

    horiz * depth
}
