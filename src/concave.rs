use std::collections::{BinaryHeap, HashSet};

use nalgebra::Point2 as Point;

use crate::{HullScalar, edge::Edge, segment_intersect::edges_intersect};

/// Inner logic for the concave hull functions
///
/// Unlike the wrappers, this function is generic, letting us handle f32/f64 precision properly.
/// If parry provided versions of the convex hull function that worked on other scalar types,
/// then we could support those too, possibly entirely using generics.
#[inline]
pub(crate) fn concave_hull_inner<T: HullScalar>(
    points: &[Point<T>],
    concavity: T,
    convex_hull: Vec<usize>,
) -> Vec<(usize, Point<T>)> {
    if points.len() <= 3 {
        // Degenerate case with enough points for a convex hull, but too few points to make a concave hull
        // Just return the convex hull
        return convex_hull.into_iter().map(|id| (id, points[id])).collect();
    }

    // Heap up the convex edges by length
    let mut edge_heap = BinaryHeap::with_capacity(convex_hull.len());
    let mut boundary_points = HashSet::with_capacity(convex_hull.len());
    for id in 0..convex_hull.len() {
        let i = convex_hull[id];
        let j = convex_hull[(id + 1) % convex_hull.len()];

        boundary_points.insert(i);
        edge_heap.push(Edge::new(i, j, points));
    }

    // Start opening the gift
    let concavity = concavity.powi(2); // Square the concavity limit to make the comparisons slightly faster
    let mut concave_hull: Vec<Edge<T>> = Vec::with_capacity(convex_hull.len());

    'edges: while let Some(edge) = edge_heap.pop() {
        // TODO: scale this check based on local density?
        // It's in the original paper, but *not* in the JS impl...
        if edge.norm_squared() > concavity {
            // This edge is long enough that we should try to split it

            // Find the best point to add in the middle
            // TODO: use a BVH to make this not slow as hell
            let mut best: Option<(usize, &Point<T>, T)> = None;
            'points: for (i, p) in points.iter().enumerate() {
                if i == edge.i || i == edge.j {
                    // Do not consider points that are already on the edge
                    continue 'points;
                }
                let e1 = p - edge.point_i;
                let e2 = edge.point_j - p;
                let e_v = edge.point_j - edge.point_i;

                let angle = e_v.angle(&e1).max(e_v.angle(&e2));
                if best.as_ref().map(|best| best.2 > angle).unwrap_or(true) {
                    best = Some((i, p, angle));
                }
            }
            let best = best.expect("Point cloud should have at least one point");

            // Check boundary to avoid creating a degenerate polygon
            // Note: The original paper recommends adding a check to make sure the angle is less than 90 degrees.
            //       I did a ton of testing and I could not find a single case where this made a difference
            //       in the final hull, even though the check was hit multiple times.
            //       So, I ommitted it for performance.
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

        sorted_hull.push((curr.i, curr.point_i));
        curr = next;
    }
    sorted_hull.push((curr.i, curr.point_i));

    sorted_hull
}
