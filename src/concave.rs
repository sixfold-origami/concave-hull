use std::collections::{BinaryHeap, HashSet};

use crate::parry2d::{
    bounding_volume::{Aabb, BoundingVolume},
    shape::{Compound, SharedShape},
};
use crate::{Point, Real};
use nalgebra::Isometry2 as Isometry;

use crate::{edge::Edge, segment_intersect::edges_intersect};

/// Computes the concave hull of the provided point cloud, using the provided concavity parameter
///
/// Inputs:
/// - `points`: A list of points, making up the point cloud to generate the concave hull for.
/// It is assumed that this list contains no repeat points.
/// - `concavity`: A parameter determining how concave the hull should be.
/// See the crate-level docs for guidance on picking the concavity parameter.
///
/// The returned [`Vec`] contains a tuple of:
/// - The index of the hull point in the original slice
/// - The value of the point in the original slice
///
/// The points are returned in counter-clockwise order.
#[inline]
pub fn concave_hull(points: &[Point], concavity: Real) -> Vec<(usize, Point)> {
    if points.len() <= 1 {
        // Degenerate case with too few points to make a convex hull
        // Just return the original point (or nothing)
        return points.iter().enumerate().map(|(id, p)| (id, *p)).collect();
    }

    // Get the convex hull from parry
    let convex_hull = parry2d::transformation::convex_hull_idx(points);

    if points.len() <= 3 {
        // Degenerate case with enough points for a convex hull, but too few points to make a concave hull
        // Just return the convex hull
        return convex_hull.into_iter().map(|id| (id, points[id])).collect();
    }

    // Build compound of points
    let point_compound = Compound::new(
        points
            .iter()
            .map(|p| (Isometry::translation(p.x, p.y), SharedShape::ball(0.1)))
            .collect(),
    );
    let point_qbvh = point_compound.qbvh();
    let max_window_size = point_compound.local_aabb().extents().max() * 0.6;
    let window_step_size = max_window_size / 6.;

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
    let mut concave_hull: Vec<Edge<Real>> = Vec::with_capacity(convex_hull.len());

    'edges: while let Some(edge) = edge_heap.pop() {
        // TODO: scale this check based on local density?
        // It's in the original paper, but *not* in the JS impl...
        if edge.norm_squared() > concavity {
            // This edge is long enough that we should try to split it

            // Find the best point to add in the middle
            // Slowly expand the window until we find something
            // TODO: We might be able to do better with a custom visitor
            let mut best: Option<(usize, &Point, Real)> = None;
            let mut window = Aabb::new(
                edge.point_i.inf(&edge.point_j),
                edge.point_i.sup(&edge.point_j),
            );
            let mut candidates = Vec::new();

            while window.extents().min() < max_window_size && best.is_none() {
                point_qbvh.intersect_aabb(&window, &mut candidates);
                'candidates: for candidate in candidates.drain(0..) {
                    let i = candidate as usize;
                    if i == edge.i || i == edge.j {
                        // Do not consider points that are already on the edge
                        continue 'candidates;
                    }

                    let p = &points[i];
                    let e1 = p - edge.point_i;
                    let e2 = edge.point_j - p;
                    let e_v = edge.point_j - edge.point_i;

                    let angle = e_v.angle(&e1).max(e_v.angle(&e2));
                    if best.as_ref().map(|best| best.2 > angle).unwrap_or(true) {
                        best = Some((i, p, angle));
                    }
                }

                if best.is_none() {
                    window.loosen(window_step_size);
                }
            }

            let Some(best) = best else {
                continue 'edges;
            };

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
