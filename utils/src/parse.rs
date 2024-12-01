use crate::matrix::Matrix;
use std::str::FromStr;

/// The general parsing type. Is implemented for `String`s.
pub trait Parseable {
    fn to_matrix<T: Clone + FromStr>(&self) -> Result<Matrix<T>, <T as FromStr>::Err>;

    /// Parses a string into a matrix of type `T`.
    fn to_matrix_chars<T: Clone, E>(
        &self,
        f: impl FnMut(char) -> Result<T, E>,
    ) -> Result<Matrix<T>, E>;

    /// Parses a string into a vector of type `T`.
    fn to_vec<T, E>(&self, f: impl FnMut(&str) -> Result<T, E>) -> Result<Vec<T>, E>;
}

impl Parseable for &str {
    /// Reads a matrix of input, separated by whitespace, into some other type.
    fn to_matrix<T: Clone + FromStr>(&self) -> Result<Matrix<T>, <T as FromStr>::Err> {
        Ok(Matrix::from_2d_vec(
            self.lines()
                .map(|x| {
                    x.split_whitespace()
                        .map(|v| v.parse())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<Vec<_>>, _>>()?,
        ))
    }

    /// Reads a matrix of input, specifically, a matrix of characters, into some other type.
    fn to_matrix_chars<T: Clone, E>(
        &self,
        mut f: impl FnMut(char) -> Result<T, E>,
    ) -> Result<Matrix<T>, E> {
        Ok(Matrix::from_2d_vec(
            self.lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(&mut f).collect::<Result<Vec<_>, _>>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    /// Reads a series of lines as input into a vector of some type.
    fn to_vec<T, E>(&self, f: impl FnMut(&str) -> Result<T, E>) -> Result<Vec<T>, E> {
        self.lines().filter(|l| !l.is_empty()).map(f).collect()
    }
}
