use {std::sync::Arc, std::time::Instant, tokio::sync::Mutex};

const NUM_THREAD: usize = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit: usize = 10_000_000_000;
    macro_rules! compute {
        ($index: expr, $offset: expr) => {{
            let i = (3 + 4 * (NUM_THREAD * $index + $offset)) as f64;
            2.0f64 / (i * (i - 2.0))
        }};
    }

    // https://tokio.rs/tokio/tutorial/shared-state
    let pi = Arc::new(Mutex::new(0.0f64));
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
        "async/await limit: {limit} => {} in {} msec.",
        *pi.lock().await,
        (end - start).as_millis(),
    );
    Ok(())
}
