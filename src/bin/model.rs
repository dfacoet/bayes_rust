use fugue::runtime::interpreters::PriorHandler;
use fugue::{Trace, runtime};
use rand::SeedableRng;
use rand::rngs::StdRng;

use bayes_rust::random_model;

fn main() {
    let model = random_model();
    let mut rng = StdRng::seed_from_u64(42);

    let handler = PriorHandler {
        rng: &mut rng,
        trace: Trace::default(),
    };

    let (value, trace) = runtime::handler::run(handler, model);
    println!("Value: {:?}", value);
    println!("Log probability: {:.4}", trace.total_log_weight());
    println!("Trace: {:?}", trace);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_runs() {
        main();
    }
}
