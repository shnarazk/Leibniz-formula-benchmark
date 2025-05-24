#[cfg(feature = "parallel")]
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let beg = Instant::now();
    let pairs: u32 = 100_000_000;
    #[cfg(feature = "parallel")]
    let seq = (0..=pairs).into_par_iter();
    #[cfg(not(feature = "parallel"))]
    let seq = 0..=pairs;

    let val = seq
        .map(|j| {
            let i = (j as f64) * 4.0;
            2.0 / ((i + 1.0) * (i + 3.0))
        })
        .sum::<f64>();

    let end = Instant::now();
    println!(
        "{} pairs: {pairs} => {} in {} sec",
        if cfg!(feature = "parallel") {
            "parallel, "
        } else {
            ""
        },
        val * 4.0,
        (end - beg).as_secs_f64(),
    );
}
