use rand::Rng;
use rand::rngs::ThreadRng;

pub const INFINITY: f32 = f32::INFINITY;

thread_local! {
    static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::thread_rng());
}

pub fn random_generator() -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn random_generator_range(min: f32, max: f32) -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
}

pub fn linear_to_gamma(x: f32) -> f32 {
    if x > 0.0 {
        return x.sqrt();
    }
    return 0.0
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    pub fn generate_numbers() {
        let a: f32 = random_generator();
        assert!(0.0 <= a && a <= 1.0);
    }
}