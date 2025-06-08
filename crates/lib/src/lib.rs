use std::collections::{BinaryHeap, HashSet};

use parry2d::{
    math::{Isometry, Point as ParryPoint},
    query::intersection_test,
    transformation::convex_hull_idx,
};

mod edge;

pub use edge::Edge;
pub use parry2d;
pub type Point = ParryPoint<f32>;

pub fn concave_hull(points: &[Point], concavity: f32) -> Vec<Edge> {
    // Get the convex hull from parry
    let convex = convex_hull_idx(points);

    // Heap up the convex edges by length
    let mut edge_heap = BinaryHeap::with_capacity(convex.len());
    for idx in 0..convex.len() {
        edge_heap.push(Edge::from_points_and_idx(points, idx, convex.len()));
    }

    // Start opening the gift
    let concavity = concavity.powi(2); // Square the concavity limit to make the comparisons slightly faster
    let mut boundary_points: HashSet<usize> = HashSet::with_capacity(convex.len());
    let mut concave_hull: Vec<Edge> = Vec::with_capacity(convex.len());

    while let Some(edge) = edge_heap.pop() {
        // TODO: scale this check based on local density
        if edge.norm_squared() > concavity {
            // This edge is long enough that we should try to split it

            // Find the best point to add in the middle
            // TODO: use a BVH to make this not slow as hell
            let mut best: Option<(usize, &Point, f32)> = None;
            for (i, p) in points.iter().enumerate() {
                let e1 = edge.segment.a - p; // Only comparing angles, so order doesn't matter here
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
                // Note: Unsure if we should also check edges in the heap
                if concave_hull.iter().any(|edge| {
                    intersection_test(
                        &Isometry::default(),
                        &edge.segment,
                        &Isometry::default(),
                        &e1.segment,
                    )
                    .expect("Segments can be intersected")
                        || intersection_test(
                            &Isometry::default(),
                            &edge.segment,
                            &Isometry::default(),
                            &e2.segment,
                        )
                        .expect("Segments can be intersected")
                }) {
                    edge_heap.push(e1);
                    edge_heap.push(e2);
                    boundary_points.insert(best.0);
                    continue;
                }
            }
        }

        concave_hull.push(edge);
    }

    // TODO: Sort them so they're in CCW order
    concave_hull
}
