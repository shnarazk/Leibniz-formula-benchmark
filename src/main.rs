use rug::Rational;

fn main() {
    let limit = 200_000;
    let mut curr_sum: Rational = Rational::from((1i64, 1i64));
    let mut denominator: usize = 3;
    let mut adding = false;
    for _ in 0..limit {
        if adding {
            curr_sum += Rational::from((1i64, denominator));
        } else {
            curr_sum -= Rational::from((1i64, denominator));
        }
        denominator += 2;
        adding = !adding;
    }
    println!("limit:{} => {}", limit, 4.0 * curr_sum.to_f64());
}
