use std::collections::HashSet;
use utils::{
    itertools::Itertools, macros::solution, matrix::Matrix, parse::Parseable, point::Point,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[solution(day = 6, part = 1)]
fn part_1(input: &str) -> usize {
    let m = input.to_matrix_chars(Ok::<_, usize>).unwrap();
    let mut guard = (Facing::Up, m.find_index(|c| *c == '^').unwrap());
    let mut pointset = HashSet::from([guard.1]);

    while guard.1.x > 0 && guard.1.x < m.width() - 1 && guard.1.y > 0 && guard.1.y < m.height() - 1
    {
        guard = step(&m, guard);
        pointset.insert(guard.1);
    }

    pointset.len()
}

#[solution(day = 6, part = 2)]
fn part_2(input: &str) -> usize {
    let m = input.to_matrix_chars(Ok::<_, usize>).unwrap();
    let mut guard = (Facing::Up, m.find_index(|c| *c == '^').unwrap());
    let mut visited = vec![guard];

    while guard.1.x > 0 && guard.1.x < m.width() - 1 && guard.1.y > 0 && guard.1.y < m.height() - 1
    {
        guard = step(&m, guard);
        visited.push(guard);
    }

    let mut obstructions = 0;

    for (i, (f, p)) in visited.iter().enumerate().skip(1) {
        let candidates = visited[0..i - 1].iter().filter(|(_, p2)| p == p2).count();

        println!("({:?}, {}) {}", f, p, candidates);

        obstructions += candidates;
    }

    obstructions
}

fn step(m: &Matrix<char>, guard: (Facing, Point<usize>)) -> (Facing, Point<usize>) {
    match guard.0 {
        Facing::Up | Facing::Down => {
            if guard.0 == Facing::Up {
                if *m.get(guard.1 - (0, 1).into()) == '#' {
                    (Facing::Right, guard.1)
                } else {
                    (Facing::Up, guard.1 - (0, 1).into())
                }
            } else if *m.get(guard.1 + (0, 1).into()) == '#' {
                (Facing::Left, guard.1)
            } else {
                (Facing::Down, guard.1 + (0, 1).into())
            }
        }
        Facing::Left | Facing::Right => {
            if guard.0 == Facing::Right {
                if *m.get(guard.1 + (1, 0).into()) == '#' {
                    (Facing::Down, guard.1)
                } else {
                    (Facing::Right, guard.1 + (1, 0).into())
                }
            } else if *m.get(guard.1 - (1, 0).into()) == '#' {
                (Facing::Up, guard.1)
            } else {
                (Facing::Left, guard.1 - (1, 0).into())
            }
        }
    }
}
