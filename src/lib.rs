//! # concave-hull
//!
//! `concave-hull` is an implementation of the [gift opening concave hull algorithm](https://deeplearning.lipingyang.org/wp-content/uploads/2019/07/Project-10-report_Implementation-of-a-fast-and-efficient-concave-hull-algorithm.pdf), written in Rust.
//!
//! The top level export is a function called `concave_hull`.
//! See the docs for that function for details on usage, or check the example at `examples/basic.rs`
//!
//! ## Choosing the concavity parameter
//!
//! Concave hulls are a somewhat subjective thing.
//! While it's possible to generate a concave hull which minimizes the area of the final polygon, this is often undesirable, as it leads to very crinkly shapes.
//! To remedy this, a concavity parameter is exposed, which controls how tight the final concave hull is around the point cloud.
//! In general, you should pick a concavity parameter which produces "desirable" results on your datasets, whatever that means for your application.
//! Here is some guidance:
//! - The concavity parameter ranges from zero to positive infinity
//! - `0` produces a maximally crinkly shape
//! - `+inf` prevents any concavity, returning the convex hull of the point cloud
//! - `40` is usually a good starting point
//!
//! Note that the concavity parameter **is not scale invariant**.
//! This means that a point cloud which covers an area from 0 to 100 will need a smaller concavity parameter than an equivalent point cloud that covers an area from 0 to 1000.
//!
//! ## Features
//!
//! This crate has two features for precision:
//! - `f32` (default feature): Enables `f32`-precision versions of the concave hull computation and relevant re-exports (an `f32`-precision point, for example)
//! - `f64`: Enables `f64`-precision versions of the concave hull computation and relevant re-exports (an `f64`-precision point, for example)
//!
//! If neither feature is enabled, then this crate has no public exports.
//! Enabling both simultaneously is supported (cargo features must be purely additive), with relevant functions being exported under the `f32` or `f64` submodules, respectively.
//!
//! This crate has one additional feature, `benches`, which is only used for benchmarks.
//! End users of this library should never enable it.

#![warn(missing_docs)]
#![feature(trait_alias)]

use nalgebra::{RealField, Scalar};
use num_traits::float::TotalOrder;

mod concave;
mod edge;
mod segment_intersect;

#[cfg(feature = "benches")]
pub use edge::Edge;
#[cfg(feature = "benches")]
pub use segment_intersect::edges_intersect;

/// Trait bound for scalars we can work with
///
/// In practice, I think this is just the float types
#[cfg(not(feature = "benches"))]
pub(crate) trait HullScalar = Scalar + RealField + Copy + TotalOrder;

/// Trait bound for scalars we can work with
///
/// In practice, I think this is just the float types
#[cfg(feature = "benches")]
pub trait HullScalar = Scalar + RealField + Copy + TotalOrder;

/// Spatial points and concave hull generation for [`prim@f32`] precision
#[cfg(feature = "f32")]
pub mod f32 {
    /// [`parry2d`]'s point type, which [`concave_hull`] uses internally for all its math
    ///
    /// This is also the point type used in function signatures and returns
    pub type Point = parry2d::math::Point<f32>;
    pub use parry2d;

    use crate::concave::concave_hull_inner;

    /// Computes the concave hull of the provided point cloud, using the provided concavity parameter
    ///
    /// Inputs:
    /// - `points`: A list of points, making up the point cloud to generate the concave hull for.
    /// It is assumed that this list contains no repeat points.
    /// - `concavity`: A parameter determining how concave the hull should be.
    ///
    /// See the crate-level docs for guidance on picking the concavity parameter.
    /// The returned [`Vec`] contains a tuple of:
    /// - The index of the hull point in the original slice
    /// - The value of the point in the original slice
    ///
    /// The points are returned in counter-clockwise order.
    pub fn concave_hull(points: &[Point], concavity: f32) -> Vec<(usize, Point)> {
        if points.len() <= 1 {
            // Degenerate case with too few points to make a convex hull
            // Just return the original point (or nothing)
            return points.iter().enumerate().map(|(id, p)| (id, *p)).collect();
        }

        // Get the convex hull from parry
        let convex = parry2d::transformation::convex_hull_idx(points);

        concave_hull_inner(points, concavity, convex)
    }
}

/// Spatial points and concave hull generation for [`prim@f64`] precision
#[cfg(feature = "f64")]
pub mod f64 {
    /// [`parry2d`]'s point type, which [`concave_hull`] uses internally for all its math
    ///
    /// This is also the point type used in function signatures and returns
    pub type Point = parry2d::math::Point<f64>;
    pub use parry2d_f64 as parry2d;

    use crate::concave::concave_hull_inner;

    /// Computes the concave hull of the provided point cloud, using the provided concavity parameter
    ///
    /// Inputs:
    /// - `points`: A list of points, making up the point cloud to generate the concave hull for.
    /// It is assumed that this list contains no repeat points.
    /// - `concavity`: A parameter determining how concave the hull should be.
    ///
    /// See the crate-level docs for guidance on picking the concavity parameter.
    /// The returned [`Vec`] contains a tuple of:
    /// - The index of the hull point in the original slice
    /// - The value of the point in the original slice
    ///
    /// The points are returned in counter-clockwise order.
    pub fn concave_hull(points: &[Point], concavity: f64) -> Vec<(usize, Point)> {
        if points.len() <= 1 {
            // Degenerate case with too few points to make a convex hull
            // Just return the original point (or nothing)
            return points.iter().enumerate().map(|(id, p)| (id, *p)).collect();
        }

        // Get the convex hull from parry
        let convex = parry2d::transformation::convex_hull_idx(points);

        concave_hull_inner(points, concavity, convex)
    }
}

#[cfg(test)]
mod tests {
    use super::f32::*;

    mod small_clouds {
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

        #[test]
        fn zero_points() {
            let hull = concave_hull(&POINTS[0..0], 10.);
            assert_eq!(hull, Vec::new());
        }

        #[test]
        fn one_point() {
            let hull = concave_hull(&POINTS[0..1], 10.);
            assert_eq!(hull, Vec::from([(0, POINTS[0])]));
        }

        #[test]
        fn two_points() {
            let hull = concave_hull(&POINTS[0..2], 10.);
            assert_eq!(hull, Vec::from([(0, POINTS[0]), (1, POINTS[1])]));
        }

        #[test]
        fn three_points() {
            let hull = concave_hull(&POINTS[0..3], 10.);
            assert_eq!(
                hull,
                Vec::from([(0, POINTS[0]), (2, POINTS[2]), (1, POINTS[1]),])
            );
        }

        #[test]
        fn square() {
            let hull = concave_hull(&[POINTS[1], POINTS[2], POINTS[4], POINTS[5]], 10.);
            assert_eq!(
                hull,
                Vec::from([
                    (2, POINTS[4]),
                    (0, POINTS[1]),
                    (1, POINTS[2]),
                    (3, POINTS[5]),
                ])
            );
        }
    }

    mod question_mark {
        use std::fs::File;

        use csv::ReaderBuilder;

        use super::*;

        fn load_question_mark() -> Vec<Point> {
            let f = File::open("./test_data/question_mark.csv").unwrap();

            let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);

            reader
                .records()
                .map(|r| {
                    let r = r.unwrap();
                    let x = r[0].parse().unwrap();
                    let y = r[1].parse().unwrap();

                    Point::new(x, y)
                })
                .collect()
        }

        #[test]
        fn reasonable_concave() {
            let points = load_question_mark();
            let hull = concave_hull(&points, 40.);

            let expected = Vec::from([
                (16, Point::new(187.0, 87.0)),
                (17, Point::new(173.0, 97.0)),
                (24, Point::new(177.0, 180.0)),
                (1, Point::new(182.0, 201.0)),
                (20, Point::new(179.0, 225.0)),
                (27, Point::new(182.0, 245.0)),
                (31, Point::new(187.0, 270.0)),
                (32, Point::new(204.0, 306.0)),
                (81, Point::new(221.0, 332.0)),
                (42, Point::new(248.0, 361.0)),
                (41, Point::new(243.0, 388.0)),
                (79, Point::new(247.0, 406.0)),
                (47, Point::new(240.0, 425.0)),
                (49, Point::new(228.0, 447.0)),
                (50, Point::new(211.0, 466.0)),
                (59, Point::new(192.0, 473.0)),
                (60, Point::new(156.0, 481.0)),
                (62, Point::new(128.0, 483.0)),
                (71, Point::new(100.0, 474.0)),
                (70, Point::new(80.0, 456.0)),
                (72, Point::new(60.0, 461.0)),
                (74, Point::new(34.0, 446.0)),
                (75, Point::new(32.0, 410.0)),
                (76, Point::new(53.0, 396.0)),
                (67, Point::new(78.0, 400.0)),
                (66, Point::new(100.0, 408.0)),
                (55, Point::new(134.0, 420.0)),
                (54, Point::new(165.0, 415.0)),
                (43, Point::new(177.0, 378.0)),
                (38, Point::new(179.0, 347.0)),
                (35, Point::new(158.0, 333.0)),
                (34, Point::new(145.0, 299.0)),
                (28, Point::new(141.0, 274.0)),
                (22, Point::new(134.0, 230.0)),
                (2, Point::new(141.0, 208.0)),
                (23, Point::new(143.0, 185.0)),
                (0, Point::new(162.0, 168.0)),
                (5, Point::new(160.0, 100.0)),
                (4, Point::new(141.0, 92.0)),
                (9, Point::new(134.0, 70.0)),
                (10, Point::new(126.0, 53.0)),
                (11, Point::new(139.0, 34.0)),
                (12, Point::new(160.0, 29.0)),
                (14, Point::new(182.0, 34.0)),
                (15, Point::new(192.0, 58.0)),
            ]);

            assert_eq!(hull, expected);
        }

        #[test]
        fn maximally_concave() {
            let points = load_question_mark();
            let hull = concave_hull(&points, 0.);

            let expected = Vec::from([
                (21, Point::new(163.0, 208.0)),
                (26, Point::new(162.0, 219.0)),
                (20, Point::new(179.0, 225.0)),
                (3, Point::new(158.0, 236.0)),
                (27, Point::new(182.0, 245.0)),
                (31, Point::new(187.0, 270.0)),
                (29, Point::new(156.0, 265.0)),
                (30, Point::new(173.0, 293.0)),
                (80, Point::new(187.0, 320.0)),
                (32, Point::new(204.0, 306.0)),
                (36, Point::new(190.0, 335.0)),
                (37, Point::new(206.0, 355.0)),
                (81, Point::new(221.0, 332.0)),
                (40, Point::new(221.0, 362.0)),
                (42, Point::new(248.0, 361.0)),
                (41, Point::new(243.0, 388.0)),
                (79, Point::new(247.0, 406.0)),
                (47, Point::new(240.0, 425.0)),
                (46, Point::new(219.0, 410.0)),
                (45, Point::new(196.0, 418.0)),
                (49, Point::new(228.0, 447.0)),
                (48, Point::new(218.0, 439.0)),
                (51, Point::new(197.0, 449.0)),
                (50, Point::new(211.0, 466.0)),
                (59, Point::new(192.0, 473.0)),
                (58, Point::new(173.0, 466.0)),
                (60, Point::new(156.0, 481.0)),
                (57, Point::new(153.0, 456.0)),
                (63, Point::new(138.0, 464.0)),
                (62, Point::new(128.0, 483.0)),
                (61, Point::new(119.0, 468.0)),
                (71, Point::new(100.0, 474.0)),
                (64, Point::new(100.0, 442.0)),
                (70, Point::new(80.0, 456.0)),
                (72, Point::new(60.0, 461.0)),
                (69, Point::new(61.0, 437.0)),
                (74, Point::new(34.0, 446.0)),
                (73, Point::new(43.0, 429.0)),
                (75, Point::new(32.0, 410.0)),
                (77, Point::new(60.0, 418.0)),
                (76, Point::new(53.0, 396.0)),
                (78, Point::new(66.0, 401.0)),
                (67, Point::new(78.0, 400.0)),
                (68, Point::new(83.0, 422.0)),
                (66, Point::new(100.0, 408.0)),
                (65, Point::new(112.0, 427.0)),
                (56, Point::new(124.0, 442.0)),
                (55, Point::new(134.0, 420.0)),
                (53, Point::new(155.0, 430.0)),
                (52, Point::new(179.0, 435.0)),
                (54, Point::new(165.0, 415.0)),
                (44, Point::new(179.0, 401.0)),
                (39, Point::new(204.0, 386.0)),
                (43, Point::new(177.0, 378.0)),
                (38, Point::new(179.0, 347.0)),
                (35, Point::new(158.0, 333.0)),
                (33, Point::new(165.0, 311.0)),
                (34, Point::new(145.0, 299.0)),
                (28, Point::new(141.0, 274.0)),
                (22, Point::new(134.0, 230.0)),
                (2, Point::new(141.0, 208.0)),
                (23, Point::new(143.0, 185.0)),
                (25, Point::new(163.0, 189.0)),
                (0, Point::new(162.0, 168.0)),
                (5, Point::new(160.0, 100.0)),
                (4, Point::new(141.0, 92.0)),
                (19, Point::new(153.0, 87.0)),
                (8, Point::new(155.0, 75.0)),
                (9, Point::new(134.0, 70.0)),
                (10, Point::new(126.0, 53.0)),
                (7, Point::new(151.0, 58.0)),
                (11, Point::new(139.0, 34.0)),
                (12, Point::new(160.0, 29.0)),
                (14, Point::new(182.0, 34.0)),
                (13, Point::new(167.0, 53.0)),
                (15, Point::new(192.0, 58.0)),
                (6, Point::new(177.0, 70.0)),
                (16, Point::new(187.0, 87.0)),
                (18, Point::new(168.0, 75.0)),
                (17, Point::new(173.0, 97.0)),
                (24, Point::new(177.0, 180.0)),
                (1, Point::new(182.0, 201.0)),
            ]);

            assert_eq!(hull, expected);
        }

        #[test]
        fn minimally_concave() {
            let points = load_question_mark();
            let hull = concave_hull(&points, f32::INFINITY);

            let expected = Vec::from([
                (50, Point::new(211.0, 466.0)),
                (59, Point::new(192.0, 473.0)),
                (60, Point::new(156.0, 481.0)),
                (62, Point::new(128.0, 483.0)),
                (71, Point::new(100.0, 474.0)),
                (72, Point::new(60.0, 461.0)),
                (74, Point::new(34.0, 446.0)),
                (75, Point::new(32.0, 410.0)),
                (10, Point::new(126.0, 53.0)),
                (11, Point::new(139.0, 34.0)),
                (12, Point::new(160.0, 29.0)),
                (14, Point::new(182.0, 34.0)),
                (15, Point::new(192.0, 58.0)),
                (42, Point::new(248.0, 361.0)),
                (79, Point::new(247.0, 406.0)),
                (47, Point::new(240.0, 425.0)),
                (49, Point::new(228.0, 447.0)),
            ]);

            assert_eq!(hull, expected);
        }
    }
}
