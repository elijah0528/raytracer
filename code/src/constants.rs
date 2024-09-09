use rand::Rng;
use rand::rngs::ThreadRng;

pub struct RandomGenerator {
    rng: ThreadRng
}

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.141592;

pub fn degrees_to_radians (degrees: f32) -> f32 {
    degrees * PI / 180.0
}

thread_local! {
    static RNG: std::cell::RefCell<ThreadRng> = std::cell::RefCell::new(rand::thread_rng());
}

pub fn random_generator() -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn random_generator_range(min: f32, max: f32) -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
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