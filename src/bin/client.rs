use resonate::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    let n: u64 = env::args()
        .nth(1)
        .unwrap_or_else(|| "5".to_string())
        .parse()
        .expect("input must be a non-negative integer");

    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("factorial-client".into()),
        ..Default::default()
    });

    let promise_id = format!("factorial-{n}");

    let result: u64 = resonate
        .rpc(&promise_id, "factorial", n)
        .target("poll://any@factorial-workers")
        .await
        .expect("rpc to worker failed");

    println!("Factorial of {n} is {result}");
}
