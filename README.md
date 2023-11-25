# A mirco benchmark with pi

This program has been used for inter-language comparisons.

See:
- Pharo and Python: [https://github.com/shnarazk/learn-pharo/blob/master/README.md](https://github.com/shnarazk/learn-pharo/blob/master/README.md) 
- BQN: [https://github.com/shnarazk/learn-bqn/blob/main/pi-benchmark](https://github.com/shnarazk/learn-bqn/blob/main/pi-benchmark/README.md)

# Old data

## A performance comparison on x86_64 (core i5 2018)

| program             | 100M pairs |    1G pairs   |
|---------------------|-----------:|--------------:|
| BQN (SIMD)          |       1.48 | out of memory |
| Pharo11 (naive)     |       1.16 |         11.72 |
| Python3.10 (naive)  |      16.00 |        168.67 |
| Rust (naive)        |       0.09 |          0.62 |
| Rust (custom)       |   0.013(+) |      0.136(+) |
| Scratch(turbo mode) |     227.26 |    don't know |

- 1 pair is a positive term and the following negative term
- (+) wall-clock
