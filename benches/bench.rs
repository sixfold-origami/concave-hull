use std::{fs::File, time::Duration};

use concave_hull::{Point, concave_hull};
use criterion::{Criterion, criterion_group, criterion_main};
use csv::ReaderBuilder;

fn load_data(path: &str) -> Vec<Point> {
    let f = File::open(path).unwrap();

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

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    group
        .measurement_time(Duration::from_secs_f32(60.))
        .sample_size(1000);

    let polygon = load_data("./test_data/polygon.csv");
    group.bench_function("polygon", |b| b.iter(|| concave_hull(&polygon, 40.)));

    let question_mark = load_data("./test_data/question_mark.csv");
    group.bench_function("question mark", |b| {
        b.iter(|| concave_hull(&question_mark, 40.))
    });

    let concaveman_1k = load_data("./test_data/concaveman_1k.csv");
    group.bench_function("concaveman_1k", |b| {
        b.iter(|| concave_hull(&concaveman_1k, 150.))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
