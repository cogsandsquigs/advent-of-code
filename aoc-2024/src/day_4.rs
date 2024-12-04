use std::iter::repeat_n;

use utils::{itertools::Itertools, macros::solution, matrix::Matrix, parse::Parseable};

#[solution(day = 4, part = 1)]
fn part_1(input: &str) -> usize {
    let m = input.to_matrix_chars(Ok::<char, usize>).unwrap();
    let finding = ["XMAS", "SAMX"];
    let mut total = 0;

    for line in m
        .columns()
        .iter()
        .chain(m.rows().iter())
        .chain(m.square_diagonals_right_to_left().iter())
        .chain(m.square_diagonals_left_to_right().iter())
    {
        let str_line: String = line.iter().copied().collect();

        total += str_line.match_indices(finding[0]).count();
        total += str_line.match_indices(finding[1]).count();
    }

    total
}

#[solution(day = 4, part = 2)]
fn part_2(input: &str) -> usize {
    let m = input.to_matrix_chars(Ok::<char, usize>).unwrap();

    let possible_chars = ['X', 'M', 'A', 'S'];
    let possible_submatrices = [
        Matrix::from_2d_vec(vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ]),
        Matrix::from_2d_vec(vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ]),
        Matrix::from_2d_vec(vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ]),
        Matrix::from_2d_vec(vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ]),
    ];
    let mut every_submatrix = vec![];

    for sm in possible_submatrices {
        for combo in repeat_n(possible_chars, 4)
            .multi_cartesian_product()
            .unique()
        {
            let mut i = 0;
            let mut vs = vec![];

            for v in sm.get_all() {
                if *v == '.' {
                    vs.push(combo[i]);
                    i += 1;
                } else {
                    vs.push(*v);
                }
            }

            let m = Matrix::from_vec(vs, sm.width(), sm.height());

            every_submatrix.push(m)
        }
    }

    let mut total = 0;

    for sm in every_submatrix {
        total += m.count_submatrices(sm);
    }

    total
}
