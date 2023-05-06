use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use rug::Rational;

fn bench_parallels(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallel Pi calculation");
    for i in [100_000i32, 200_000, 400_000, 800_000].iter() {
        group.bench_with_input(BenchmarkId::new("sum", i), i, |b, i| {
            b.iter(|| pi_asaw0(*i))
        });
        // group.bench_with_input(BenchmarkId::new("reduce", i), i, |b, i| { b.iter(|| pi_asaw2(*i)) });
    }
    group.finish();
}

criterion_group!(benches, bench_parallels);
criterion_main!(benches);

pub fn pi_asaw0(limit: i32) -> f64 {
    let result = (0i32..=limit)
        .map(|i| {
            if i % 2 == 0 {
                1.0f64 / (2 * i + 1) as f64
            } else {
                1.0f64 / (-2 * i - 1) as f64
            }
        })
        // .map(|denominator| Rational::from((1i64, denominator)))
        // .sum::<Rational>()
        .sum::<f64>();
    4.0 * result
}
