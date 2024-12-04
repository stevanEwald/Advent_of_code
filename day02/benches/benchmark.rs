use aoc::{part_1, part_2};
use criterion::{criterion_group, criterion_main, Criterion};
use std::env;
use std::fs;
use std::hint::black_box;

pub fn part_1_bench(c: &mut Criterion) {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input.txt";
    let input = fs::read_to_string(path).unwrap();

    c.bench_function("part_1", |b| b.iter(|| part_1(black_box(&input))));
}

pub fn part_2_bench(c: &mut Criterion) {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input.txt";
    let input = fs::read_to_string(path).unwrap();

    c.bench_function("part_2", |b| b.iter(|| part_2(black_box(&input))));
}

criterion_group!(benches, part_1_bench, part_2_bench);
criterion_main!(benches);
