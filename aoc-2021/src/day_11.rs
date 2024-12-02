use std::collections::HashSet;
use utils::{itertools::Itertools, macros::solution, parse::Parseable, point::Point};

#[solution(day = 11, part = 1)]
fn part_2(input: &str) -> usize {
    let mut matrix = input
        .to_matrix_chars(|c| c.to_string().parse::<usize>())
        .unwrap();

    let mut step = 0;

    loop {
        step += 1;

        matrix = matrix.map(|x| x + 1);

        let mut did_flash: HashSet<Point<usize>> = HashSet::new();

        loop {
            let tmp = matrix.clone();
            let vs = tmp
                .into_iter()
                .filter(|(_, &x)| x > 9)
                .filter(|(p, _)| !did_flash.contains(p))
                .collect_vec();

            // Break if we finish the flashing
            if vs.is_empty() {
                break;
            }

            vs.iter().for_each(|(p, _)| {
                did_flash.insert(*p);
            });

            let ns = vs
                .iter()
                .map(|(p, _)| p)
                .flat_map(|p| p.neighbors())
                .filter(|p| p.x < matrix.width() && p.y < matrix.height())
                .collect_vec(); // NOTE: Gotta collect it all here 2 avoid referencing issues

            for n in ns {
                matrix.set(n, matrix.get(n) + 1);
            }
        }

        for p in did_flash {
            matrix.set(p, 0);
        }

        if matrix.all(|x| *x == 0) {
            break;
        }
    }

    step
}

#[solution(day = 11, part = 1)]
fn part_1(input: &str) -> usize {
    let mut matrix = input
        .to_matrix_chars(|c| c.to_string().parse::<usize>())
        .unwrap();

    let mut flashes = 0;

    for _ in 0..100 {
        matrix = matrix.map(|x| x + 1);

        let mut did_flash: HashSet<Point<usize>> = HashSet::new();

        loop {
            let tmp = matrix.clone();
            let vs = tmp
                .into_iter()
                .filter(|(_, &x)| x > 9)
                .filter(|(p, _)| !did_flash.contains(p))
                .collect_vec();

            // Break if we finish the flashing
            if vs.is_empty() {
                break;
            }

            flashes += vs.len();
            vs.iter().for_each(|(p, _)| {
                did_flash.insert(*p);
            });

            let ns = vs
                .iter()
                .map(|(p, _)| p)
                .flat_map(|p| p.neighbors())
                .filter(|p| p.x < matrix.width() && p.y < matrix.height())
                .collect_vec(); // NOTE: Gotta collect it all here 2 avoid referencing issues

            for n in ns {
                matrix.set(n, matrix.get(n) + 1);
            }
        }

        for p in did_flash {
            matrix.set(p, 0);
        }
    }

    flashes
}
