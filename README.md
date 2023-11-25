# A mirco benchmark with Leibniz formula

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

| Language/approach   | Implementation | 100M pairs |   1G pairs | note |
|---------------------|----------------|-----------:|-----------:|------|
| BQN (naive)         | CBQN 0.3+ o3n  |            |            |      |
| BQN (SIMD)          | CBQN 0.3+ o3n  |            |            |      |
| Pharo (naive)       | Pharo 11       |            |            |      |
| Python (naive)      | CPython 3.11   |            |            |      |
| Rust (naive)        | Rust 1.74      |            |            |      |
| Rust (threads)      | Rust 1.74      |            |            |      |
| Scratch(naive)      | Scratch (turbo)|            |            |      |
| Swift(naive)        | Swift 5.9.0    |            |            |      |
| Uiua (naive)        | Uiua 0.3.1+    |            |            |      |
| Uiua (threads)      | Uiua 0.3.1+    |            |            |      |


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
