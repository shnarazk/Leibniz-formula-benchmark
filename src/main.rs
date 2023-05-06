#[cfg(feature = "bignum")]
use rug::Rational;
use {std::sync::Arc, std::time::Instant, tokio::sync::Mutex};

const NUM_THREAD: usize = 4;

// #[cfg(feature = "bignum")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit: usize = 10_000_000_000;
    macro_rules! compute {
        ($index: expr, $offset: expr) => {{
            let i = (3 + 4 * (NUM_THREAD * $index + $offset)) as f64;
            2.0f64 / (i * (i - 2.0))
        }};
    }
    // #[cfg(feature = "bignum")]
    // let val = seq
    //     .map(|j| {
    //         let demoninator = j * 4;
    //         Rational::from((1, denominator + 1)) - Rational::from((1, denominator + 3))
    //     })
    //     .sum::<Rational>()
    //     .to_f64();

    // https://tokio.rs/tokio/tutorial/shared-state
    let pi = Arc::new(Mutex::new(0.0f64));
    // #[cfg(not(feature = "bignum"))]
    let start = Instant::now();
    let handles = (0..NUM_THREAD)
        .map(|i| {
            let pi = pi.clone();
            tokio::spawn(async move {
                let sub = (0..limit / (2 * NUM_THREAD))
                    .map(|k| compute!(k, i))
                    .sum::<f64>();
                *pi.lock().await += 4.0 * sub;
            })
        })
        .collect::<Vec<_>>();
    for i in handles {
        assert!(i.await.is_ok());
    }
    let end = Instant::now();
    println!(
        "{} limit: {limit} => {} in {} msec.",
        if cfg!(feature = "bignum") {
            "bignum, async/await"
        } else {
            "async/await"
        },
        *pi.lock().await,
        (end - start).as_millis(),
    );
    Ok(())
}
