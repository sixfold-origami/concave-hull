use std::cmp::Ordering;

use parry2d::shape::Segment;

use crate::Point;

#[derive(Debug, Clone)]
pub struct Edge {
    pub i: usize,
    pub j: usize,
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
    pub fn from_points_and_idx(points: &[Point], idx: usize, total_edges: usize) -> Self {
        let j = (idx + 1) % total_edges;
        let p = points[idx];
        let next = points[j];

        Self {
            i: idx,
            j,
            segment: Segment::new(p, next),
        }
    }

    #[inline]
    pub fn norm_squared(&self) -> f32 {
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
