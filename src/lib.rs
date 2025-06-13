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
//! ## Features
//!
//! This crate has one feature, `benches`, which is only used for benchmarks.
//! End users of this library should never enable it.

#![warn(missing_docs)]

use std::collections::{BinaryHeap, HashSet};

use parry2d::{math::Point as ParryPoint, transformation::convex_hull_idx};

#[cfg(feature = "benches")]
pub use edge::Edge;
#[cfg(not(feature = "benches"))]
use edge::Edge;
#[cfg(feature = "benches")]
pub use segment_intersect::edges_intersect;
#[cfg(not(feature = "benches"))]
use segment_intersect::edges_intersect;

mod edge;
mod segment_intersect;

/// [`parry2d`]'s point type, which [`concave_hull`] uses internally for all its math
///
/// This is also the point type used in function signatures and returns
pub type Point = ParryPoint<f32>;
pub use parry2d;

/// Computes the concave hull of the provided point cloud, using the provided concavity parameter
///
/// Inputs:
/// - `points`: A list of points, making up the point cloud to generate the concave hull for.
/// It is assumed that this list contains no repeat points.
/// - `concavity`: A parameter determining how concave the hull should be.
/// See the crate-level docs for guidance on picking the concavity parameter.
/// The returned [`Vec`] contains a tuple of:
/// - The index of the hull point in the original slice
/// - The value of the point in the original slice
///
/// The points are returned in counter-clockwise order
pub fn concave_hull(points: &[Point], concavity: f32) -> Vec<(usize, Point)> {
    // TODO: Add special cases and tests for point clouds with fewer than three points
    // Get the convex hull from parry
    let convex = convex_hull_idx(points);

    // Heap up the convex edges by length
    let mut edge_heap = BinaryHeap::with_capacity(convex.len());
    let mut boundary_points = HashSet::with_capacity(convex.len());
    for id in 0..convex.len() {
        let i = convex[id];
        let j = convex[(id + 1) % convex.len()];

        boundary_points.insert(i);
        edge_heap.push(Edge::new(i, j, points));
    }

    // Start opening the gift
    let concavity = concavity.powi(2); // Square the concavity limit to make the comparisons slightly faster
    let mut concave_hull: Vec<Edge> = Vec::with_capacity(convex.len());

    'edges: while let Some(edge) = edge_heap.pop() {
        // TODO: scale this check based on local density?
        // It's in the original paper, but *not* in the JS impl...
        if edge.norm_squared() > concavity {
            // This edge is long enough that we should try to split it

            // Find the best point to add in the middle
            // TODO: use a BVH to make this not slow as hell
            let mut best: Option<(usize, &Point, f32)> = None;
            'points: for (i, p) in points.iter().enumerate() {
                if i == edge.i || i == edge.j {
                    // Do not consider points that are already on the edge
                    continue 'points;
                }
                let e1 = p - edge.segment.a;
                let e2 = edge.segment.b - p;
                let e_v = edge.segment.scaled_direction();

                let angle = e_v.angle(&e1).max(e_v.angle(&e2));
                if best.as_ref().map(|best| best.2 > angle).unwrap_or(true) {
                    best = Some((i, p, angle));
                }
            }
            let best = best.expect("Point cloud should have at least one point");

            // Check boundary to avoid creating a degenerate polygon
            // TODO: add an option to check that the angle is less than pi/2
            if !boundary_points.contains(&best.0) {
                let (e1, e2) = edge.split_by(*best.1, best.0);

                // Check if the new edges would intersect any existing ones
                // TODO: BVH might be faster? Hard to say given how frequently we'd be adding new segments
                if concave_hull
                    .iter()
                    .chain(edge_heap.iter())
                    .all(|edge| !(edges_intersect(edge, &e1) || edges_intersect(edge, &e2)))
                {
                    edge_heap.push(e1);
                    edge_heap.push(e2);
                    boundary_points.insert(best.0);
                    continue 'edges;
                }
            }
        }

        concave_hull.push(edge);
    }

    // Sort the edges in the hull end to end
    // TODO: Can we get clever with pointer shenanigans to maintain this as we build the hull?
    let mut sorted_hull = Vec::with_capacity(concave_hull.len());
    let mut curr = concave_hull
        .pop() // Start with an arbitrary edge
        .expect("Concave hull has at least one point");

    while !concave_hull.is_empty() {
        // Walk the pointers, grabbing edges in order
        let next = concave_hull
            .iter()
            .position(|edge| edge.i == curr.j)
            .expect("Concave hull is well-formed");
        let next = concave_hull.swap_remove(next);

        sorted_hull.push((curr.i, curr.segment.a));
        curr = next;
    }
    sorted_hull.push((curr.i, curr.segment.a));

    sorted_hull
}
