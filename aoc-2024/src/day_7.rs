use std::iter::repeat_n;

use utils::{itertools::Itertools, macros::solution};

#[solution(day = 7, part = 1)]
fn part_1(input: &str) -> u128 {
    let ps = parse_input(input);
    let mut res = 0;

    for (sol, ns) in ps {
        let numops = ns.len() - 1;
        let mut did_solve = false;

        for ops in repeat_n(["+", "*"].iter().copied(), numops).multi_cartesian_product() {
            if eval(&ns, &ops) == sol {
                did_solve = true;
                break;
            }
        }

        if did_solve {
            res += sol;
        }
    }

    res
}

#[solution(day = 7, part = 2)]
fn part_2(input: &str) -> u128 {
    let ps = parse_input(input);
    let mut res = 0;

    for (sol, ns) in ps {
        let numops = ns.len() - 1;
        let mut did_solve = false;

        for ops in repeat_n(["+", "*", "||"].iter().copied(), numops).multi_cartesian_product() {
            if eval(&ns, &ops) == sol {
                did_solve = true;
                break;
            }
        }

        if did_solve {
            res += sol;
        }
    }

    res
}

fn eval(ns: &[u128], ops: &[&str]) -> u128 {
    let mut res = ns[0];

    for (i, n) in ns[1..].iter().enumerate() {
        match ops[i] {
            "+" => res += n,
            "*" => res *= n,
            "||" => res = (res.to_string() + &n.to_string()).parse().unwrap(), // (res * 10u128.pow((*n as f64).log10().ceil() as u32)) + n,
            _ => unreachable!(),
        }
    }

    res
}

fn parse_input(input: &str) -> Vec<(u128, Vec<u128>)> {
    let mut ret = vec![];
    for l in input.lines() {
        let mut i = l.split(':');
        let (sol, nstr) = (
            i.next().unwrap().parse::<u128>().unwrap(),
            i.next().unwrap(),
        );
        let ns = nstr
            .split_whitespace()
            .map(|s| s.parse::<u128>().unwrap())
            .collect();

        ret.push((sol, ns))
    }

    ret
}
