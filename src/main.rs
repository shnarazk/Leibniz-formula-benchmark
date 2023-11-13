fn main() {
    let limit: u64 = 1_000_000_000;
    let seq = 0..=limit;
    let val = seq
        .map(|j| {
            let i = j * 4;
            2.0 / (((i + 1) * (i + 3)) as f64)
        })
        .sum::<f64>();
    println!("imit: {limit} => {}", val * 4.0);
}
