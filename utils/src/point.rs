use core::fmt::{Display, Formatter};
use itertools::Itertools;
use num::{traits::SaturatingSub, Float, Num, Signed};
use std::{
    hash::Hash,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

/// A point in a 2D grid.
/// TODO: Change points to be T instead of usize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Num,
{
    /// Returns a new `Point`.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// Public API for Point.
impl<T> Point<T>
where
    T: Num + SaturatingSub + Hash + Eq,
{
    /// Returns the set of all neighbors of the given coordinates. The neighbors are returned in the
    /// order of left, right, up, down, top-left, top-right, bottom-left, bottom-right.
    pub fn neighbors(&self) -> Vec<Self>
    where
        T: Copy + From<usize>,
    {
        [
            Self::new(self.x.saturating_sub(&1.into()), self.y),
            Self::new(self.x + 1.into(), self.y),
            Self::new(self.x, self.y.saturating_sub(&1.into())),
            Self::new(self.x, self.y + 1.into()),
            Self::new(
                self.x.saturating_sub(&1.into()),
                self.y.saturating_sub(&1.into()),
            ),
            Self::new(self.x + 1.into(), self.y.saturating_sub(&1.into())),
            Self::new(self.x.saturating_sub(&1.into()), self.y + 1.into()),
            Self::new(self.x + 1.into(), self.y + 1.into()),
        ]
        .iter()
        .unique()
        .filter(|x| **x != *self)
        .cloned()
        .collect_vec()
    }

    /// Returns the set of all neighbors orthogonal to the given coordinates.
    /// The neighbors are returned in the order of left, right, up, down.
    pub fn orthogonal_neighbors(&self) -> Vec<Self>
    where
        T: From<usize> + Copy,
    {
        [
            Self::new(self.x.saturating_sub(&1.into()), self.y),
            Self::new(self.x + 1.into(), self.y),
            Self::new(self.x, self.y.saturating_sub(&1.into())),
            Self::new(self.x, self.y + 1.into()),
        ]
        .iter()
        .unique()
        .filter(|x| **x != *self)
        .cloned()
        .collect_vec()
    }

    /// Returns the set of all neighbors diagonal to the given coordinates.
    /// The neighbors are returned in the order of top-left, top-right, bottom-left, bottom-right.
    pub fn diagonal_neighbors(&self) -> Vec<Self>
    where
        T: From<usize> + Copy,
    {
        vec![
            Self::new(
                self.x.saturating_sub(&1.into()),
                self.y.saturating_sub(&1.into()),
            ),
            Self::new(self.x + 1.into(), self.y.saturating_sub(&1.into())),
            Self::new(self.x.saturating_sub(&1.into()), self.y + 1.into()),
            Self::new(self.x + 1.into(), self.y + 1.into()),
        ]
    }
}

impl<T> Point<T>
where
    T: Signed + Copy,
{
    /// Computes the Manhattan distance between two points.
    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T> Point<T>
where
    T: Float,
{
    /// Computes the Manhattan distance between two points.
    pub fn euclidean_distance(&self, other: &Self) -> T {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl<T> Add for Point<T>
where
    T: Num + Add,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Point<T>
where
    T: Num + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Point<T>
where
    T: Num + Sub,
{
    type Output = Point<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Point<T>
where
    T: Num + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Num + SubAssign + Clone,
{
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for (T, T)
where
    T: Num,
{
    fn from(point: Point<T>) -> Self {
        (point.x, point.y)
    }
}

impl From<Point<usize>> for Point<isize> {
    fn from(value: Point<usize>) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
}

impl<T> Display for Point<T>
where
    T: Num + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distances() {
        let p1: Point<_> = (1, 2).into();
        let p2: Point<_> = (5, 7).into();

        assert_eq!(p1.manhattan_distance(&p2), 9);

        let p1: Point<_> = (1., 2.).into();
        let p2: Point<_> = (5., 7.).into();

        assert_eq!(p1.euclidean_distance(&p2), (41.).sqrt());
    }
}
