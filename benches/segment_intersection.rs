use std::time::Duration;

use concave_hull::{Edge, Point, edges_intersect};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

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

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("segment_intersection");
    group
        .measurement_time(Duration::from_secs_f32(15.))
        .sample_size(1000);

    let e1 = Edge::new(0, 1, &POINTS);
    let e2 = Edge::new(0, 1, &POINTS);
    group.bench_function("same edge", |b| {
        b.iter(|| edges_intersect(black_box(&e1), black_box(&e2)))
    });

    let e1 = Edge::new(0, 1, &POINTS);
    let e2 = Edge::new(1, 4, &POINTS);
    group.bench_function("connected edges", |b| {
        b.iter(|| edges_intersect(black_box(&e1), black_box(&e2)))
    });

    let e1 = Edge::new(1, 9, &POINTS);
    let e2 = Edge::new(3, 7, &POINTS);
    group.bench_function("intersection x", |b| {
        b.iter(|| edges_intersect(black_box(&e1), black_box(&e2)))
    });

    let e1 = Edge::new(1, 7, &POINTS);
    let e2 = Edge::new(4, 6, &POINTS);
    group.bench_function("intersection t", |b| {
        b.iter(|| edges_intersect(black_box(&e1), black_box(&e2)))
    });

    let e1 = Edge::new(1, 9, &POINTS);
    let e2 = Edge::new(4, 8, &POINTS);
    group.bench_function("parallel", |b| {
        b.iter(|| edges_intersect(black_box(&e1), black_box(&e2)))
    });
}

criterion_group!(segment_intersection, criterion_benchmark);
criterion_main!(segment_intersection);
