# A mirco benchmark with Leibniz formula

This program has been used for inter-language comparisons.

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
