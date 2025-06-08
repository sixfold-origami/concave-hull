use parry2d::{math::Point as ParryPoint, transformation::convex_hull};

pub type Point = ParryPoint<f32>;

pub fn concave_hull(points: &[Point]) -> Vec<Point> {
    convex_hull(points)
}
