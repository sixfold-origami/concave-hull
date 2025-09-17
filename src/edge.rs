use std::cmp::Ordering;
use {nalgebra::Point2 as Point, nalgebra::Vector2 as Vector};

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

impl Edge<f32> {
    pub(crate) fn linearized_angle(&self) -> f32 {
        let e_v = self.point_j - self.point_i;
        debug_assert!(e_v != Vector::zeros(), "Edge has identical endpoints!");

        let mut offset = 0.;
        if e_v.y < 0. {
            offset += 2.;

            if e_v.x >= 0.0 {
                offset += 1.;
            }
        } else if e_v.y > 0. {
            if e_v.x < 0.0 {
                offset += 1.;
            }
        } else {
            if e_v.x <= 0.0 { return 2. } else { return 0. }
        }

        let slope = (e_v.y / e_v.x).tanh();

        if slope >= 0. {
            offset + slope
        } else {
            offset + 1. + slope
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::f32::Point;

    use super::*;

    /// An array of points in a numpad grid, in numpad order
    ///
    /// 7 8 9
    /// 4 5 6
    /// 1 2 3
    /// 0
    const POINTS: [Point; 10] = [
        Point::new(0., 0.),
        Point::new(0., 1.),
        Point::new(1., 1.),
        Point::new(2., 1.),
        Point::new(0., 2.),
        Point::new(1., 2.),
        Point::new(2., 2.),
        Point::new(0., 3.),
        Point::new(1., 3.),
        Point::new(2., 3.),
    ];

    /// Tests a bunch of edges in CCW order, and ensures that the linearized angle strictly increases
    /// All four cardinal directions are hit, as are the 4 diagonals, and one point between each of these 8 points
    #[test]
    fn ccw_ordering_test() {
        let edges = [
            Edge::new(5, 6, &POINTS), // Right
            Edge::new(1, 6, &POINTS),
            Edge::new(5, 9, &POINTS), // Up right
            Edge::new(2, 9, &POINTS),
            Edge::new(5, 8, &POINTS), // Up
            Edge::new(3, 8, &POINTS),
            Edge::new(5, 7, &POINTS), // Up left
            Edge::new(6, 7, &POINTS),
            Edge::new(5, 4, &POINTS), // Left
            Edge::new(9, 4, &POINTS),
            Edge::new(5, 1, &POINTS), // Down left
            Edge::new(8, 1, &POINTS),
            Edge::new(5, 2, &POINTS), // Down
            Edge::new(7, 2, &POINTS),
            Edge::new(5, 3, &POINTS), // Down right
            Edge::new(4, 3, &POINTS),
        ];

        edges
            .iter()
            .for_each(|e| println!("{:?}: {}", e, e.linearized_angle())); // Debugging

        assert!(edges.iter().map(|e| e.linearized_angle()).is_sorted())
    }

    #[test]
    fn right() {
        assert_eq!(Edge::new(5, 6, &POINTS).linearized_angle(), 0.);
    }

    #[test]
    fn up() {
        assert_eq!(Edge::new(5, 8, &POINTS).linearized_angle(), 1.);
    }

    #[test]
    fn left() {
        assert_eq!(Edge::new(5, 4, &POINTS).linearized_angle(), 2.);
    }

    #[test]
    fn down() {
        assert_eq!(Edge::new(5, 2, &POINTS).linearized_angle(), 3.);
    }
}
