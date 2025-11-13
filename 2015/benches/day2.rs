use aoc_2015::day2::*;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_part1(c: &mut Criterion) {
    let mut group = c.benchmark_group(BENCH_GROUPS[0]);
    group.bench_function("baseline", |b| b.iter(|| part1_baseline(black_box(INPUT))));
    group.bench_function("optimize", |b| b.iter(|| part1(black_box(INPUT))));
    group.bench_function("lut", |b| b.iter(|| part1_lut(black_box(INPUT))));
    group.finish();
}

fn bench_part2(c: &mut Criterion) {
    let mut group = c.benchmark_group(BENCH_GROUPS[1]);
    group.bench_function("baseline", |b| b.iter(|| part2_baseline(black_box(INPUT))));
    group.bench_function("optimize", |b| b.iter(|| part2(black_box(INPUT))));
    group.finish();
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
