use std::cmp::Ordering;

use parry2d::shape::Segment;

use crate::Point;

/// Helper struct for edges in the hull
#[derive(Debug, Clone)]
pub struct Edge {
    /// Index of the first point
    pub i: usize,
    /// Index of the second point
    pub j: usize,
    /// Segment of the edge (containing the actual values of the two points)
    pub segment: Segment,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        // Only need to check indices
        self.i == other.i && self.j == other.j
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Edges are always compared based on their length
        // We only care about relative length, so the squared norm is acceptable here
        self.norm_squared().total_cmp(&other.norm_squared())
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    /// Constructs a new [`Self`] from a list of points and two (ordered) indices into that list
    pub fn new(i: usize, j: usize, points: &[Point]) -> Self {
        Self {
            i,
            j,
            segment: Segment::new(points[i], points[j]),
        }
    }

    #[inline]
    pub(crate) fn norm_squared(&self) -> f32 {
        self.segment.scaled_direction().norm_squared()
    }

    /// Splits self in two by inserting `point` in the middle of the edge
    pub fn split_by(&self, point: Point, idx: usize) -> (Self, Self) {
        let e1 = Self {
            i: self.i,
            j: idx,
            segment: Segment::new(self.segment.a, point),
        };
        let e2 = Self {
            i: idx,
            j: self.j,
            segment: Segment::new(point, self.segment.b),
        };

        (e1, e2)
    }
}
