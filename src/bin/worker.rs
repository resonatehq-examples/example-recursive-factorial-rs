use resonate::prelude::*;

/// A recursive factorial workflow.
///
/// Each invocation pairs with a durable promise whose ID is `factorial-{n}`.
/// Once a given `n` has been computed, the promise is RESOLVED and any later
/// invocation with the same ID returns the cached result without recomputing.
///
/// The recursive step uses `ctx.rpc` with a `target` of
/// `poll://any@factorial-workers`, so the next call can land on any worker in
/// the group. Run multiple workers and watch the calculation spread across
/// them.
#[resonate::function]
async fn factorial(ctx: &Context, n: u64) -> Result<u64> {
    println!("Calculating factorial({n})");
    if n <= 1 {
        return Ok(1);
    }

    let result: u64 = ctx
        .rpc("factorial", n - 1)
        .target("poll://any@factorial-workers")
        .await?;

    Ok(n * result)
}

#[tokio::main]
async fn main() {
    let resonate = Resonate::new(ResonateConfig {
        url: Some("http://localhost:8001".into()),
        group: Some("factorial-workers".into()),
        ..Default::default()
    });

    resonate.register(factorial).unwrap();

    println!("factorial worker running...");
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c");
}
