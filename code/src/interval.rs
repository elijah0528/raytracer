use crate::constants::{INFINITY};

pub struct Interval {
    min: f32,
    max: f32
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn empty () -> Self {
        Interval { min: INFINITY, max: -INFINITY }
    }

    pub fn universe () -> Self {
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

    pub fn surrounds (&self, x: f32) -> bool {
        self.min < x && x < self.max
    }


}

impl Default for Interval {
    fn default() -> Interval {
        Interval::universe()
    }
}