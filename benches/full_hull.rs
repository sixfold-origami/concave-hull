use std::{fs::File, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use csv::ReaderBuilder;
use geo::{ConcaveHull, MultiPoint};

fn load_data(path: &str) -> MultiPoint {
    let f = File::open(path).unwrap();

    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);

    let points: Vec<_> = reader
        .records()
        .map(|r| {
            let r = r.unwrap();
            let x = r[0].parse().unwrap();
            let y = r[1].parse().unwrap();

            (x, y)
        })
        .collect();

    MultiPoint::from(points)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_hull");
    group
        .measurement_time(Duration::from_secs_f32(60.))
        .sample_size(1000);

    let polygon = load_data("./test_data/polygon.csv");
    group.bench_function("polygon", |b| b.iter(|| polygon.concave_hull(0.4)));

    let question_mark = load_data("./test_data/question_mark.csv");
    group.bench_function("question mark", |b| {
        b.iter(|| question_mark.concave_hull(0.2))
    });

    let concaveman_1k = load_data("./test_data/concaveman_1k.csv");
    group.bench_function("concaveman_1k", |b| {
        b.iter(|| concaveman_1k.concave_hull(0.001))
    });
}

criterion_group!(full_hull, criterion_benchmark);
criterion_main!(full_hull);
