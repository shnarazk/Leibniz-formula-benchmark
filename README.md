# A mirco benchmark with Leibniz formula

- [wikipedia](https://en.wikipedia.org/wiki/Leibniz_formula_for_π)

## Program structure

- Use float numbers to evaluate each term.
- Pairing a positive term and the following negative term to reduce the number of division.

#### outline

```rust
fn pi(limit: u32) {
  let mut sum: f64 = 0.0
  for i in 0..limit {                 // 0, 1, 2, 3, ... limit-1 (limit items)
    let k: f64 = 4.0 * (i as f64);    // 0, 4, 8, 12, ...        (limit items)
    sum += 2.0 / ((k + 1) * (k + 3)); // == 1/(k+1) - 1/(k+3)    (limit pairs)
  }
  4.0 * sum
}
```

# Results

- CPU: Apple M3
- Memory: 24GB

| Language/approach   | Implementation |  10M pairs | 100M [sec] |       note |
|---------------------|----------------|-----------:|-----------:|------------|
| BQN (naive)         | CBQN 0.3+ o3n  |            |            |            |
| BQN (SIMD)          | CBQN 0.3+ o3n  |            |            |            |
| Factor (naive)      | 0.99+          |            |      0.137 | Core i5 2.3GHz 2cores |
| Koka (naive)        | 3.0.4          |            |      0.08  | 2024-02-11 |
| Lean4 (naive)       | 4.12.0         |      0.011 |      0.089 | 2024-10-03 |
| Pharo (naive)       | Pharo 11+      |      0.167 |      1.141 | 2024-01-01 |
| Python (naive)      | CPython 3.12   |      1.036 |     10.435 | 2023-11-25 |
| Rust (naive)        | Rust 1.74      |      0.010 |      0.081 | 2023-11-25 |
| Rust (threads)      | Rust 1.81      |      0.002 |      0.015 | 2024-10-03 |
| Scratch(naive)      | Scratch (turbo)|            |            |            |
| Swift(naive)        | Swift 5.9.0    |      0.015 |      0.100 | 2023-11-25 |
| Uiua (naive)        | Uiua 0.3.1+    |      3.110 |     31.669 | 2023-11-30 |
| Uiua (threads)      | Uiua 0.3.1+    |      1.410 |     13.728 | 2023-11-30 |


# Old data

## A performance comparison on x86_64 (core i5 2018)

| program             | 100M pairs |    1G pairs   |
|---------------------|-----------:|--------------:|
| BQN (naive)         |       5.32 |         53.95 |
| BQN (SIMD)          |       0.21 |          2.09 |
| Pharo11 (naive)     |       1.16 |         11.72 |
| Python3.10 (naive)  |      16.00 |        168.67 |
| Rust (naive)        |       0.09 |          0.62 |
| Rust (custom)       |   0.013(+) |      0.136(+) |
| Scratch(turbo mode) |     227.26 |    don't know |

- 1 pair is a positive term and the following negative term
- (+) wall-clock
