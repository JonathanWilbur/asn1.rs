use std::{hint::black_box, str::FromStr};
use criterion::{criterion_group, criterion_main, Criterion};
use asn1::DATE;

fn date_from_str() {
    let d1 = DATE::from_str(black_box("2022-04-11")).unwrap();
    let d2 = DATE::from_num_str(black_box("20220411")).unwrap();
    assert_eq!(d1, d2);
}

fn date_to_str() {
    let d1 = DATE::new(2022, 04, 11);
    let d2 = DATE::new(2024, 12, 30);
    let _ = d1.to_string();
    let _ = d2.to_string();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("make date from strings", |b| b.iter(|| date_from_str()));
    c.bench_function("convert date to string", |b| b.iter(|| date_to_str()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
