use std::collections::HashSet;

use utils::{itertools::Itertools, macros::solution, parse::Parseable, point::Point};

#[solution(day = 8, part = 1)]
fn part_1(input: &str) -> usize {
    let mut nodeset = HashSet::new();

    let m = input
        .to_matrix_chars(|n| {
            if n != '.' {
                nodeset.insert(n);
            }
            Ok::<_, usize>(n)
        })
        .unwrap();

    nodeset
        .iter()
        .flat_map(|n| {
            m.find_all_index(|n2| n2 == n)
                .iter()
                .copied()
                .map(|p| Point::new(p.x as isize, p.y as isize))
                .combinations(2)
                .collect_vec()
        })
        .flat_map(|v| {
            let a = v[0];
            let b = v[1];
            vec![(b - a + b), (a - b + a)]
        })
        .unique()
        .filter(|p| {
            (0..m.width() as isize).contains(&p.x) && (0..m.height() as isize).contains(&p.y)
        })
        .count()
}

#[solution(day = 8, part = 2)]
fn part_2(input: &str) -> usize {
    let mut nodeset = HashSet::new();

    let m = input
        .to_matrix_chars(|n| {
            if n != '.' {
                nodeset.insert(n);
            }
            Ok::<_, usize>(n)
        })
        .unwrap();

    nodeset
        .iter()
        .flat_map(|n| {
            m.find_all_index(|n2| n2 == n)
                .iter()
                .copied()
                .map(Point::<isize>::from)
                .combinations(2)
                .collect_vec()
        })
        .flat_map(|v| {
            let a = v[0];
            let b = v[1];
            (-((m.width() * m.height()) as isize)..(m.width() * m.height()) as isize)
                .map(move |n| b + (b - a) * n)
        })
        .unique()
        .filter(|p| {
            (0..m.width() as isize).contains(&p.x) && (0..m.height() as isize).contains(&p.y)
        })
        .count()
}
