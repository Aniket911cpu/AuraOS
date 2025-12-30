// core/animator.rs
use std::time::Instant;
use cgmath::Vector3;

/// Represents a value being animated with spring physics.
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub stiffness: f32, // 'k'
    pub damping: f32,   // 'b'
    pub mass: f32,      // 'm'
}

impl Spring {
    pub fn default() -> Self {
        Self {
            stiffness: 150.0,
            damping: 12.0,
            mass: 1.0,
        }
    }

    /// Tighter spring for smaller UI elements
    pub fn snappy() -> Self {
        Self {
            stiffness: 300.0,
            damping: 20.0,
            mass: 0.8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub current: Vector3<f32>,
    pub target: Vector3<f32>,
    pub velocity: Vector3<f32>,
    
    pub spring_config: Spring,
    pub last_update: Instant,
    pub is_settled: bool,
}

impl AnimationState {
    pub fn new(initial_val: Vector3<f32>) -> Self {
        Self {
            current: initial_val,
            target: initial_val,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            spring_config: Spring::default(),
            last_update: Instant::now(),
            is_settled: true,
        }
    }

    pub fn set_target(&mut self, new_target: Vector3<f32>) {
        if self.target != new_target {
            self.target = new_target;
            self.is_settled = false;
        }
    }

    /// Step the physics simulation by delta_time (in seconds)
    pub fn step(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Prevent huge jumps if thread sleeps or hangs
        let dt = dt.min(0.1); 

        if self.is_settled {
            return;
        }

        // F = -k(x - target) - b(v)
        let displacement = self.current - self.target;
        let spring_force = displacement * -self.spring_config.stiffness;
        let damping_force = self.velocity * -self.spring_config.damping;
        
        let total_force = spring_force + damping_force;
        let acceleration = total_force / self.spring_config.mass;

        self.velocity += acceleration * dt;
        self.current += self.velocity * dt;

        // Check for settling
        // If slow enough and close enough, snap to target
        if self.velocity.magnitude() < 0.01 && displacement.magnitude() < 0.001 {
            self.current = self.target;
            self.velocity = Vector3::new(0.0, 0.0, 0.0);
            self.is_settled = true;
        }
    }
}
