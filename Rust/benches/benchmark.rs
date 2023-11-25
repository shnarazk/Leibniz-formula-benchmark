use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rayon::prelude::*;

fn bench_parallels(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallel Pi calculation");
    for i in [100_000i64, 200_000, 400_000, 800_000].iter() {
        group.bench_with_input(BenchmarkId::new("sum", i), i, |b, i| b.iter(|| pi_par1(*i)));
        group.bench_with_input(BenchmarkId::new("reduce", i), i, |b, i| {
            b.iter(|| pi_par2(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parallels);
criterion_main!(benches);

pub fn pi_par1(limit: i64) -> f64 {
    let result = (0i64..=limit)
        .into_par_iter()
        .map(|i| if i % 2 == 0 { 2 * i + 1 } else { -2 * i - 1 })
        .map(|denominator| Rational::from((1i64, denominator)))
        .sum::<Rational>()
        .to_f64();
    4.0 * result
}

pub fn pi_par2(limit: i64) -> f64 {
    let result = (0i64..=limit)
        .into_par_iter()
        .map(|i| if i % 2 == 0 { 2 * i + 1 } else { -2 * i - 1 })
        .map(|denominator| Rational::from((1i64, denominator)))
        .reduce(
            || Rational::from((0i64, 1i64)),
            |sum: Rational, frac: Rational| sum + frac,
        )
        .to_f64();
    4.0 * result
}
