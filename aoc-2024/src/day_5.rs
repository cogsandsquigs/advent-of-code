use utils::{itertools::Itertools, macros::solution};

#[solution(day = 5, part = 1)]
fn part_1(input: &str) -> usize {
    let (ordering, updates) = parse(input);

    updates
        .iter()
        .filter(|u| ordered(&ordering, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

#[solution(day = 5, part = 2)]
fn part_2(input: &str) -> usize {
    let (ordering, updates) = parse(input);

    updates
        .iter()
        .filter(|u| !ordered(&ordering, u))
        .map(|u| reorder(&ordering, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn reorder(h: &[(usize, usize)], v: &[usize]) -> Vec<usize> {
    match v {
        [x] => vec![*x],
        xs => {
            let i = xs
                .iter()
                .position_max_by_key(|x| {
                    h.iter().filter(|(a, b)| a == *x && xs.contains(b)).count()
                })
                .unwrap();

            let x = xs[i];

            let mut r = xs.to_vec();
            r.remove(i);
            let mut r = reorder(h, &r);
            r.insert(0, x);
            r
        }
    }
}

fn ordered(h: &[(usize, usize)], v: &[usize]) -> bool {
    for i in 0..v.len() {
        let n = v[i];
        let (before, after) = v.split_at(i);

        for x in before {
            if !h.contains(&(*x, n)) {
                return false;
            }
        }

        // NOTE: has to be this way as after[0] == n
        for x in &after[1..] {
            if !h.contains(&(n, *x)) {
                return false;
            }
        }
    }

    true
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let [pairs_str, updates_str, ..] = input.split("\n\n").collect_vec()[..] else {
        unreachable!()
    };

    (
        pairs_str
            .split("\n")
            .map(|p| {
                (
                    p[0..2].parse::<usize>().unwrap(),
                    p[3..5].parse::<usize>().unwrap(),
                )
            })
            .collect(),
        updates_str
            .lines()
            .map(|l| l.split(',').map(|i| i.parse::<usize>().unwrap()).collect())
            .collect(),
    )
}
