use parry2d::math::Point;
use std::cmp::Ordering;

use crate::HullScalar;

/// Helper struct for edges in the hull
#[derive(Debug, Clone)]
pub struct Edge<T: HullScalar> {
    /// Index of the first point
    pub i: usize,
    /// Index of the second point
    pub j: usize,

    /// Value of the first point
    pub point_i: Point<T>,
    /// Value of the second point
    pub point_j: Point<T>,
}

impl<T: HullScalar> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        // Only need to check indices
        self.i == other.i && self.j == other.j
    }
}

impl<T: HullScalar> Eq for Edge<T> {}

impl<T: HullScalar> Ord for Edge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Edges are always compared based on their length
        // We only care about relative length, so the squared norm is acceptable here
        self.norm_squared().total_cmp(&other.norm_squared())
    }
}

impl<T: HullScalar> PartialOrd for Edge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: HullScalar> Edge<T> {
    /// Constructs a new [`Self`] from a list of points and two (ordered) indices into that list
    pub fn new(i: usize, j: usize, points: &[Point<T>]) -> Self {
        Self {
            i,
            j,
            point_i: points[i],
            point_j: points[j],
        }
    }

    #[inline]
    pub(crate) fn norm_squared(&self) -> T {
        (self.point_j - self.point_i).norm_squared()
    }

    /// Splits self in two by inserting `point` in the middle of the edge
    pub fn split_by(&self, point: Point<T>, idx: usize) -> (Self, Self) {
        let e1 = Self {
            i: self.i,
            j: idx,
            point_i: self.point_i,
            point_j: point,
        };
        let e2 = Self {
            i: idx,
            j: self.j,
            point_i: point,
            point_j: self.point_j,
        };

        (e1, e2)
    }
}
