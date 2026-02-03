use crate::vec3::{Vec3, Point3};

/// Easing functions for smooth animations
pub mod easing {
    pub fn linear(t: f32) -> f32 {
        t
    }

    pub fn ease_in_quad(t: f32) -> f32 {
        t * t
    }

    pub fn ease_out_quad(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    pub fn ease_in_out_quad(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    pub fn ease_out_bounce(t: f32) -> f32 {
        let n1 = 7.5625;
        let d1 = 2.75;

        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }

    pub fn ease_in_bounce(t: f32) -> f32 {
        1.0 - ease_out_bounce(1.0 - t)
    }
}

/// Keyframe for animation
#[derive(Clone, Copy)]
pub struct Keyframe {
    pub time: f32,
    pub position: Point3,
    pub scale: Vec3,
}

impl Keyframe {
    pub fn new(time: f32, position: Point3) -> Self {
        Keyframe {
            time,
            position,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn with_scale(time: f32, position: Point3, scale: Vec3) -> Self {
        Keyframe {
            time,
            position,
            scale,
        }
    }
}

/// Animation track for an object
pub struct AnimationTrack {
    keyframes: Vec<Keyframe>,
}

impl AnimationTrack {
    pub fn new() -> Self {
        AnimationTrack {
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        // Keep sorted by time
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }

    /// Get interpolated position and scale at time t
    pub fn sample(&self, t: f32) -> (Point3, Vec3) {
        if self.keyframes.is_empty() {
            return (Point3::default(), Vec3::new(1.0, 1.0, 1.0));
        }

        if self.keyframes.len() == 1 {
            return (self.keyframes[0].position, self.keyframes[0].scale);
        }

        // Find the two keyframes to interpolate between
        let mut prev_idx = 0;
        let mut next_idx = 0;

        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time <= t {
                prev_idx = i;
            }
            if kf.time >= t && next_idx == 0 {
                next_idx = i;
                break;
            }
        }

        // Handle edge cases
        if t <= self.keyframes[0].time {
            return (self.keyframes[0].position, self.keyframes[0].scale);
        }
        if t >= self.keyframes.last().unwrap().time {
            let last = self.keyframes.last().unwrap();
            return (last.position, last.scale);
        }

        if prev_idx == next_idx {
            return (self.keyframes[prev_idx].position, self.keyframes[prev_idx].scale);
        }

        let prev = &self.keyframes[prev_idx];
        let next = &self.keyframes[next_idx];

        // Calculate interpolation factor
        let duration = next.time - prev.time;
        let local_t = if duration > 0.0 {
            (t - prev.time) / duration
        } else {
            0.0
        };

        // Linear interpolation
        let position = prev.position + local_t * (next.position - prev.position);
        let scale = prev.scale + local_t * (next.scale - prev.scale);

        (position, scale)
    }
}

impl Default for AnimationTrack {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a bouncing ball animation track
pub fn create_bounce_animation(
    start_pos: Point3,
    floor_y: f32,
    num_bounces: i32,
    total_duration: f32,
    energy_loss: f32, // 0.0-1.0, how much energy is lost per bounce
) -> AnimationTrack {
    let mut track = AnimationTrack::new();
    
    let initial_height = start_pos.y() - floor_y;
    let mut current_height = initial_height;
    let mut current_time = 0.0;
    let mut bounce_count = 0;
    
    // Starting position
    track.add_keyframe(Keyframe::new(0.0, start_pos));
    
    while bounce_count < num_bounces && current_time < total_duration {
        // Time to fall/rise is proportional to sqrt of height (physics)
        let fall_time = (2.0 * current_height / 9.8).sqrt() * 0.5; // Scaled for visual effect
        
        // Hit the floor
        current_time += fall_time;
        if current_time > total_duration {
            break;
        }
        
        // At floor - squash
        let floor_pos = Point3::new(start_pos.x(), floor_y, start_pos.z());
        track.add_keyframe(Keyframe::with_scale(
            current_time - 0.02,
            Point3::new(start_pos.x(), floor_y + 0.1, start_pos.z()),
            Vec3::new(1.0, 1.0, 1.0),
        ));
        track.add_keyframe(Keyframe::with_scale(
            current_time,
            floor_pos,
            Vec3::new(1.3, 0.6, 1.3), // Squash
        ));
        track.add_keyframe(Keyframe::with_scale(
            current_time + 0.02,
            Point3::new(start_pos.x(), floor_y + 0.1, start_pos.z()),
            Vec3::new(0.85, 1.2, 0.85), // Stretch
        ));
        
        // Bounce up
        current_height *= (1.0 - energy_loss);
        let rise_time = (2.0 * current_height / 9.8).sqrt() * 0.5;
        
        current_time += rise_time;
        if current_time > total_duration {
            break;
        }
        
        // At peak
        let peak_pos = Point3::new(start_pos.x(), floor_y + current_height, start_pos.z());
        track.add_keyframe(Keyframe::new(current_time, peak_pos));
        
        // Fall back down
        current_time += rise_time;
        bounce_count += 1;
    }
    
    // Final resting position
    track.add_keyframe(Keyframe::new(
        total_duration,
        Point3::new(start_pos.x(), floor_y, start_pos.z()),
    ));
    
    track
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_track() {
        let mut track = AnimationTrack::new();
        track.add_keyframe(Keyframe::new(0.0, Point3::new(0.0, 0.0, 0.0)));
        track.add_keyframe(Keyframe::new(1.0, Point3::new(1.0, 1.0, 1.0)));

        let (pos, _) = track.sample(0.5);
        assert!((pos.x() - 0.5).abs() < 0.01);
        assert!((pos.y() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_bounce_animation() {
        let track = create_bounce_animation(
            Point3::new(0.0, 5.0, 0.0),
            0.5,
            3,
            2.0,
            0.3,
        );
        
        // Should have multiple keyframes
        assert!(track.keyframes.len() > 5);
    }
}
