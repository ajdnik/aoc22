use aoc22::days::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
}

fn bench_slow(c: &mut Criterion) {
    let day16 = read("input/day16/task.txt");
    c.bench_function("day16_part1", |b| {
        b.iter(|| day16::part1(&day16, 30).unwrap())
    });
    c.bench_function("day16_part2", |b| {
        b.iter(|| day16::part2(&day16, 26).unwrap())
    });

    let day19 = read("input/day19/task.txt");
    c.bench_function("day19_part1", |b| b.iter(|| day19::part1(&day19).unwrap()));
    c.bench_function("day19_part2", |b| b.iter(|| day19::part2(&day19).unwrap()));

    let day22 = read("input/day22/task.txt");
    c.bench_function("day22_part1", |b| b.iter(|| day22::part1(&day22).unwrap()));
    c.bench_function("day22_part2", |b| b.iter(|| day22::part2(&day22).unwrap()));

    let day23 = read("input/day23/task.txt");
    c.bench_function("day23_part1", |b| b.iter(|| day23::part1(&day23).unwrap()));
    c.bench_function("day23_part2", |b| b.iter(|| day23::part2(&day23).unwrap()));

    let day24 = read("input/day24/task.txt");
    c.bench_function("day24_part1", |b| b.iter(|| day24::part1(&day24).unwrap()));
    c.bench_function("day24_part2", |b| b.iter(|| day24::part2(&day24).unwrap()));
}

criterion_group!(benches, bench_slow);
criterion_main!(benches);
