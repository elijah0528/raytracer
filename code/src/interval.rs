use crate::constants::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn empty() -> Self {
        Interval { min: INFINITY, max: -INFINITY }
    }

    pub fn universe() -> Self {
        Interval { min: -INFINITY, max: INFINITY }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    /// Create interval that encloses both intervals
    pub fn surrounding(a: &Interval, b: &Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    /// Expand the interval by delta on both sides
    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    fn default() -> Interval {
        Interval::universe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_size() {
        let i = Interval::new(0.0, 5.0);
        assert_eq!(i.size(), 5.0);
    }

    #[test]
    fn test_interval_surrounding() {
        let a = Interval::new(0.0, 2.0);
        let b = Interval::new(3.0, 5.0);
        let c = Interval::surrounding(&a, &b);
        assert_eq!(c.min(), 0.0);
        assert_eq!(c.max(), 5.0);
    }

    #[test]
    fn test_interval_expand() {
        let i = Interval::new(0.0, 1.0);
        let e = i.expand(0.2);
        assert!((e.min() - (-0.1)).abs() < 1e-6);
        assert!((e.max() - 1.1).abs() < 1e-6);
    }
}
