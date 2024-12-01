use itertools::Itertools;

/// The matrix structure. Matrices are arranged in this way:
///     + x --->
///     y 1 2 3
///     | 4 5 6
///     | 7 8 9
///     v
#[derive(Debug)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    vals: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    /// Matrix from 2d vector.
    /// NOTE: They way this works, from a list like:
    /// [
    ///     [1, 2, ...],
    ///     [3, 4, ...],
    ///     ...
    /// ]
    /// You get the matrix:
    ///     [1, 2, ...]
    ///     [3, 4, ...]
    pub fn from_2d_vec(v: Vec<Vec<T>>) -> Matrix<T> {
        debug_assert!(!v.is_empty()); // TEST: Need actual values.
        debug_assert!(v.iter().all(|x| !x.is_empty())); // TEST: Need actual values
        debug_assert!(v.iter().all(|x| x.len() == v[0].len())); // TEST: All lengths should be the
                                                                // same!

        Matrix {
            width: v[0].len(),
            height: v.len(),
            vals: v.concat(),
        }
    }

    /// Transposes the matrix, returning a new one.
    pub fn transpose(&self) -> Matrix<T> {
        let mut vals = vec![vec![]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                let v = self.get(x, y);
                vals[x].push(v.clone());
            }
        }

        Matrix::from_2d_vec(vals)
    }
}

impl<T> Matrix<T> {
    /// Gets the size of the rows.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the size of the columns.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets a value from the matrix.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vals[x + y * self.width]
    }

    /// Gets the rows from the matrix.
    pub fn rows(&self) -> Vec<Vec<&T>> {
        self.vals
            .iter()
            .chunks(self.vals.len() / self.height)
            .into_iter()
            .map(|c| c.collect_vec())
            .collect_vec()
    }

    /// Gets the columns from the matrix.
    pub fn columns(&self) -> Vec<Vec<&T>> {
        let mut vals = vec![vec![]; self.width];

        for x in 0..self.width {
            for y in 0..self.height {
                let v = self.get(x, y);
                vals[x].push(v);
            }
        }

        vals
    }
}

impl<T> Eq for Matrix<T> where T: Eq {}

impl<T> PartialEq for Matrix<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.vals == other.vals
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn can_make_matrix() {
        let matrix = Matrix::from_2d_vec(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

        assert_eq!(matrix.width(), 2);
        assert_eq!(matrix.height(), 3);
    }

    #[test]
    fn can_get_rows() {
        let matrix = Matrix::from_2d_vec(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(matrix.rows(), vec![vec![&1, &2], vec![&3, &4]]);
    }

    #[test]
    fn can_get_cols() {
        let matrix = Matrix::from_2d_vec(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(matrix.columns(), vec![vec![&1, &3], vec![&2, &4]]);
    }

    #[test]
    fn can_transpose() {
        let matrix = Matrix::from_2d_vec(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);

        assert_eq!(
            matrix.transpose(),
            Matrix::from_2d_vec(vec![vec![1, 3, 5], vec![2, 4, 6]])
        );
    }
}
