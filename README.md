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
| BQN (naive)         | CBQN 0.8       |      0.58_ |      5.64_ | 2024-11-07 |
| BQN (SIMD)          | CBQN 0.8       |      0.03_ |      0.16_ | 2024-11-07 |
| Factor (naive)      | 0.99+          |            |      0.137 | Core i5 2.3GHz 2cores |
| Julia (naive)       | 1.11.2         |      0.013 |      0.083 | 2025-01-07 |
| Koka (naive)        | 3.0.4          |      0.08_ |            | 2024-02-11 |
| Lean4 (naive)       | 4.14.0-rc2     |      0.010 |      0.084 | 2024-11-07 |
| Pharo (naive)       | Pharo 11+      |      0.167 |      1.141 | 2024-01-01 |
| Python (naive)      | CPython 3.12   |      1.036 |     10.435 | 2023-11-25 |
| Rust (naive)        | Rust 1.74      |      0.010 |      0.081 | 2023-11-25 |
| Rust (threads)      | Rust 1.81      |      0.002 |      0.015 | 2024-10-03 |
| [Scratch(naive)](https://private-user-images.githubusercontent.com/997855/383764619-b7b629ef-add4-41ae-8be0-28e154cb81a5.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzA5Mzg0NDksIm5iZiI6MTczMDkzODE0OSwicGF0aCI6Ii85OTc4NTUvMzgzNzY0NjE5LWI3YjYyOWVmLWFkZDQtNDFhZS04YmUwLTI4ZTE1NGNiODFhNS5wbmc_WC1BbXotQWxnb3JpdGhtPUFXUzQtSE1BQy1TSEEyNTYmWC1BbXotQ3JlZGVudGlhbD1BS0lBVkNPRFlMU0E1M1BRSzRaQSUyRjIwMjQxMTA3JTJGdXMtZWFzdC0xJTJGczMlMkZhd3M0X3JlcXVlc3QmWC1BbXotRGF0ZT0yMDI0MTEwN1QwMDA5MDlaJlgtQW16LUV4cGlyZXM9MzAwJlgtQW16LVNpZ25hdHVyZT04ODFiNjM3ZjRiOWE4OGI1MmU5YzQ0NDdlMzc3NjkyMTdkOTZjNTM4N2Y5OTFiYzhkMThiZTM2NDcxNmJmZmU3JlgtQW16LVNpZ25lZEhlYWRlcnM9aG9zdCJ9.yhzzLKOxc_Xn8OW9lQvrjgC1vGbo3VoHP2zYEnBiWf4)      | Scratch (turbo)|      6.48  |     47.25  | 2024-11-07 |
| Swift(naive)        | Swift 5.9.0    |      0.015 |      0.100 | 2023-11-25 |
| Uiua (naive)        | Uiua 0.13.0    |      4.072 |     40.830 | 2024-11-07 |
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
