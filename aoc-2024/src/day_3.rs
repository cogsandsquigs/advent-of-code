use utils::{macros::solution, regex::Regex};

#[solution(day = 3, part = 1)]
fn part_1(input: &str) -> usize {
    let r = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    r.find_iter(input)
        .map(|m| &m.as_str()[4..m.len() - 1])
        .map(|s| s.split(','))
        .map(|i| i.map(|v| v.parse::<usize>().unwrap()).product::<usize>())
        .sum()
}

#[solution(day = 3, part = 2)]
fn part_2(input: &str) -> usize {
    let r = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();

    r.find_iter(input)
        .fold((0, true), |(s, do_mul), m| match m.as_str() {
            "do()" => (s, true),
            "don't()" => (s, false),
            v => (
                s + if do_mul {
                    v[4..v.len() - 1]
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .product::<usize>()
                } else {
                    0
                },
                do_mul,
            ),
        })
        .0
}
