use utils::{macros::solution, regex::Regex};

#[solution(day = 3, part = 1)]
fn part_1(input: &str) -> usize {
    let r = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    r.find_iter(input)
        .map(|m| &m.as_str()[4..m.len() - 1])
        .map(|s| s.split(','))
        .map(|mut i| {
            (
                i.next().unwrap().parse::<usize>().unwrap(),
                i.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[solution(day = 3, part = 2)]
fn part_2(input: &str) -> usize {
    let r = Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").unwrap();

    let mut s = 0;
    let mut do_mul = true;

    for m in r.find_iter(input) {
        match m.as_str() {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            v => {
                if do_mul {
                    let mut i = v[4..v.len() - 1].split(',');
                    s += i.next().unwrap().parse::<usize>().unwrap()
                        * i.next().unwrap().parse::<usize>().unwrap();
                }
            }
        }
    }

    s
}
