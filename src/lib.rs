use fugue::{Model, Normal, addr, pure, sample};

pub fn constant_model(value: f64) -> Model<f64> {
    pure(value)
}

pub fn random_model() -> Model<f64> {
    sample(addr!("x"), Normal::new(0.0, 1.0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_model() {
        let _model = constant_model(42.0);
    }

    #[test]
    fn test_random_model() {
        let _model = random_model();
    }
}
