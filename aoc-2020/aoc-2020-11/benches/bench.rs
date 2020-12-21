use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_common::io::read_input;
use aoc_2020_11::{parse_input, part_one, part_two};

fn criterion_benchmark(c: &mut Criterion) {
  let input = read_input("2020-11");
  c.bench_function("2020-11 parse   ", |b|
    b.iter(|| parse_input(black_box(&input))));
  let parsed = parse_input(&input);
  c.bench_function("2020-11 part one", |b|
    b.iter(|| part_one(black_box(&parsed))));
  c.bench_function("2020-11 part two", |b|
    b.iter(|| part_two(black_box(&parsed))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
