use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2015_01::{read_input, parse_input, part_one, part_two};

fn criterion_benchmark(c: &mut Criterion) {
  let input = read_input();
  c.bench_function("2015-01 parse   ", |b|
    b.iter(|| parse_input(black_box(&input))));
  let parsed = parse_input(&input);
  c.bench_function("2015-01 part one", |b|
    b.iter(|| part_one(black_box(&parsed))));
  c.bench_function("2015-01 part two", |b|
    b.iter(|| part_two(black_box(&parsed))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
