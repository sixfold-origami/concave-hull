use std::collections::BinaryHeap;

use parry2d::{math::Point as ParryPoint, shape::Segment, transformation::convex_hull_idx};

use crate::edge::Edge;

pub type Point = ParryPoint<f32>;

mod edge;

pub fn concave_hull(points: &[Point], concavity: f32) -> Vec<Point> {
    concave_hull_idx(points, concavity)
        .into_iter()
        .map(|id| points[id])
        .collect()
}

pub fn concave_hull_idx(points: &[Point], concavity: f32) -> Vec<usize> {
    // Get the convex hull from parry
    let convex = convex_hull_idx(points);

    // Heap up the convex edges by length
    let mut edge_heap = BinaryHeap::with_capacity(convex.len());
    for idx in 0..convex.len() {
        edge_heap.push(Edge::from_points_and_idx(points, idx, convex.len()));
    }

    // Start opening the gift
    let concavity = concavity.powi(2); // Square the concavity limit to make the comparisons slightly faster

    while let Some(edge) = edge_heap.pop() {
        if edge.norm_squared() > concavity {}
    }

    todo!()
}
