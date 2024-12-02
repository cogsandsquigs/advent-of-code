use utils::{itertools::Itertools, macros::solution};

#[solution(day = 2, part = 1)]
fn part_1(input: &str) -> usize {
    let m = input.lines().map(|l| {
        l.split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect_vec()
    });

    m.filter(|r| {
        let mut r2 = r.clone();
        r2.reverse();
        r.is_sorted() || r2.is_sorted()
    })
    .filter(|r| {
        r.iter().tuple_windows().all(|(a, b)| {
            let x = a.abs_diff(*b);
            (1..=3).contains(&x)
        })
    })
    .count()
}

#[solution(day = 2, part = 2)]
fn part_2(input: &str) -> usize {
    let m = input.lines().map(|l| {
        l.split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .collect_vec()
    });

    m.filter(is_day2_safe).count()
}

fn is_day2_safe(r: &Vec<usize>) -> bool {
    for i in 0..r.len() {
        let mut r2 = r.clone();
        r2.remove(i);
        let mut r3 = r2.clone();
        r3.reverse();
        if (r2.is_sorted() || r3.is_sorted())
            && r2.iter().tuple_windows().all(|(a, b)| {
                let x = a.abs_diff(*b);
                (1..=3).contains(&x)
            })
        {
            return true;
        } else {
            continue;
        }
    }

    false
}
