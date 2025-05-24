use std::{hint::black_box, str::FromStr};
use criterion::{criterion_group, criterion_main, Criterion};
use asn1::oid::OBJECT_IDENTIFIER;
use smallvec::SmallVec;

fn create_oid(n: u32) -> OBJECT_IDENTIFIER {
    let oid1 = OBJECT_IDENTIFIER::from_str("1.3.6.7.1").unwrap();
    let o = OBJECT_IDENTIFIER::from_prefix_and_arc(oid1, n).unwrap();
    o
}

fn create_oid_2(n: u32) -> OBJECT_IDENTIFIER {
    OBJECT_IDENTIFIER::try_from([ 2u32, 5, 4, 3, n ].as_slice()).unwrap()
}

fn vec_test() -> Vec<u8> {
    let mut v = Vec::with_capacity(16);
    for i in 0..14 {
        v.push(i);
    }
    v
}

// TODO: Report or study this further
/*
It seems like a SmallVec of 14 bytes is about 3x faster than the normal Vec,
but 15 bytes or more is 100x slower!
*/
fn smallvec_test() -> SmallVec<[u8; 14]>  {
    let mut v: SmallVec<[u8; 14]> = SmallVec::new();
    for i in 0..14 {
        v.push(i);
    }
    v
}

fn bench_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vec");
    group.bench_function("Normal", |b| b.iter(|| black_box(vec_test())));
    group.bench_function("SmallVec", |b| b.iter(|| black_box(smallvec_test())));
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create oid", |b| b.iter(|| black_box(create_oid(black_box(20)))));
    c.bench_function("create oid2", |b| b.iter(|| black_box(create_oid_2(black_box(20)))));
}

// criterion_group!(benches2, bench_vec);
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
// criterion_main!(benches2);
