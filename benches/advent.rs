use advent::{downloader::get_input, *};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    println!("Benchmarking day 5");
    let input = get_input(5, 2022).unwrap();
    let solution_func = solutions::y2022::day5::main;

    c.bench_function("unit", |b| {
        b.iter(|| black_box(solution_func(black_box(&input))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
