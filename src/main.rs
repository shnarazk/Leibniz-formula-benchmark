use {rayon::prelude::*, rug::Rational};

fn main() {
    let limit = 200_000;
    // let mut curr_sum: Rational = Rational::from((1i64, 1i64));
    // let mut denominator: i64 = 3;
    // let mut adding = false;
    // for _ in 0..limit {
    //     if adding {
    //         curr_sum += Rational::from((1i64, denominator));
    //     } else {
    //         curr_sum -= Rational::from((1i64, denominator));
    //     }
    //     denominator += 2;
    //     adding = !adding;
    // }
    // println!("limit:{} => {}", limit, 4.0 * curr_sum.to_f64());
    let result = (0i64..=limit)
        .into_par_iter()
        .map(|i| if i % 2 == 0 { 2 * i + 1 } else { -2 * i - 1 })
        .map(|denominator| Rational::from((1i64, denominator)))
        .reduce(
            || Rational::from((0i64, 1i64)),
            |sum: Rational, frac: Rational| sum + frac,
        );
    println!("limit:{} => {}", limit, 4.0 * result.to_f64());
}
