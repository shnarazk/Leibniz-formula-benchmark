#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "bignum")]
use rug::Rational;

// #[cfg(feature = "bignum")]
fn main() {
    let limit: u128 = 1_000_000_000;

    /// We generate two (positive and negative) terms from a single index.
    /// So halve `limit`.
    #[cfg(feature = "parallel")]
    let seq = (0..=limit / 2).into_par_iter();
    #[cfg(not(feature = "parallel"))]
    let seq = 0..=limit / 2;

    #[cfg(feature = "bignum")]
    let val = seq
        .map(|j| {
            let demoninator = j * 4;
            Rational::from((1, denominator + 1)) - Rational::from((1, denominator + 3))
        })
        .sum::<Rational>()
        .to_f64();
    #[cfg(not(feature = "bignum"))]
    let val = seq
        .map(|j| {
            let i = j * 4;
            2.0 / (((i + 1) * (i + 3)) as f64)
        })
        .sum::<f64>();

    println!(
        "{}{}limit: {limit} => {}",
        if cfg!(feature = "bignum") {
            "bignum, "
        } else {
            ""
        },
        if cfg!(feature = "parallel") {
            "parallel, "
        } else {
            ""
        },
        val * 4.0
    );
}
