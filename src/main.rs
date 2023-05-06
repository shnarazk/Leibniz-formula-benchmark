use std::sync::Arc;

#[cfg(feature = "bignum")]
use rug::Rational;
use tokio::sync::Mutex;

// #[cfg(feature = "bignum")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit: u128 = 1_000_000_000;
    let num_thread = 5;
    macro_rules! compute {
        ($index: expr, $offset: expr) => {{
            let i = (3 + 4 * (num_thread * $index + $offset)) as f64;
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
    let total = Arc::new(Mutex::new(0.0f64));
    #[cfg(not(feature = "bignum"))]
    let mut handles = Vec::new();
    for i in 0..num_thread {
        let total = total.clone();
        handles.push(tokio::spawn(async move {
            let mut sub_total: f64 = 0.0;
            for k in 0..10000 {
                sub_total += compute!(k, i);
                if k % 10000 == 0 {
                    dbg!(i, sub_total);
                    // todo!();
                }
            }
            let mut total = total.lock().await;
            *total += sub_total;
        }));
    }
    for i in handles {
        assert!(i.await.is_ok());
    }
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
        *total.lock().await * 4.0
    );
    Ok(())
}
