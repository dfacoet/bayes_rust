use fugue::*;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn main() {
    println!("🎲 Hello, Probabilistic World!");

    // Create a simple model: flip a biased coin
    let coin_model = sample(addr!("coin"), Bernoulli::new(0.7).unwrap());

    // Run the model with a seeded RNG for reproducible results
    let mut rng = StdRng::seed_from_u64(42);
    let (is_heads, trace) = runtime::handler::run(
        runtime::interpreters::PriorHandler {
            rng: &mut rng,
            trace: Trace::default(),
        },
        coin_model,
    );

    // Print the result - notice it's a bool, not a float!
    let result = if is_heads { "Heads" } else { "Tails" };
    println!("🪙 Coin flip result: {}", result);
    println!("📊 Log probability: {:.4}", trace.total_log_weight());

    // Demonstrate type safety with different distributions
    let mut rng = StdRng::seed_from_u64(123);

    // Count events - returns u64 directly
    let (event_count, _) = runtime::handler::run(
        runtime::interpreters::PriorHandler {
            rng: &mut rng,
            trace: Trace::default(),
        },
        sample(addr!("events"), Poisson::new(3.5).unwrap()),
    );
    println!("🎯 Event count: {} (type: u64)", event_count);

    // Choose category - returns usize for safe indexing
    let options: Vec<_> = vec!["red", "green", "blue"];
    let (category_idx, _) = runtime::handler::run(
        runtime::interpreters::PriorHandler {
            rng: &mut rng,
            trace: Trace::default(),
        },
        sample(addr!("color"), Categorical::uniform(3).unwrap()),
    );
    println!(
        "🎨 Chosen color: {} (safe indexing!)",
        options[category_idx]
    );

    println!("✅ Fugue is working correctly!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_runs() {
        // This test verifies that main() runs without panicking
        main();
    }
}
