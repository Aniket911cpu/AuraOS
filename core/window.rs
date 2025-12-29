// core/window.rs
// Part of the "Liquid Glass" UI System

use cgmath::{Vector3, Quaternion};
use std::time::Instant;

/// Represents the physical material properties of a UI Window.
/// This defines how the window interacts with light and force.
#[derive(Debug, Clone, Copy)]
pub struct MaterialProperties {
    /// Refractive Index (n). 1.52 for Crown Glass.
    /// Modulates how much background distortion occurs.
    pub refraction_index: f32,
    
    /// Physical density (affects impact force on docking).
    pub density: f32,
    
    /// Surface roughness for micro-grain noise (0.0 - 1.0).
    pub roughness: f32,
}

/// The fundamental UI unit in Aura OS.
/// Not just a rect, but a physical object in 3D space.
#[derive(Debug)]
pub struct Window {
    pub id: u64,
    pub title: String,
    
    // Spatial State
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub velocity: Vector3<f32>,
    
    // The "Infinite Z-Axis" elevation (0.0 is 'floor', 10.0 is 'active focus')
    pub elevation: f32,
    
    // Physical attributes
    pub mass_kg: f32,
    pub dimensions: Vector3<f32>, // Width, Height, Thickness
    pub material: MaterialProperties,
    
    // State flags
    pub is_docked: bool,
    pub last_interaction: Instant,
}

impl Window {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            elevation: 1.0, // Default floating height
            mass_kg: 0.5,   // Standard window weight
            dimensions: Vector3::new(800.0, 600.0, 0.05), // 5cm thick virtual glass
            material: MaterialProperties {
                refraction_index: 1.52,
                density: 1.0,
                roughness: 0.1,
            },
            is_docked: false,
            last_interaction: Instant::now(),
        }
    }

    /// Calculate the blurring radius required for the background
    /// based on the window's Z-elevation and thickness.
    pub fn calculate_blur_radius(&self) -> f32 {
        // Higher elevation = further from background = more blur
        // Thicker glass = more internal scattering
        let base_blur = self.elevation * 2.5;
        let thickness_factor = self.dimensions.z * 10.0;
        
        base_blur + thickness_factor
    }

    /// Apply a force vector to the window (e.g. from a user "throw" gesture).
    /// F = ma -> a = F/m
    pub fn apply_force(&mut self, force: Vector3<f32>, delta_time: f32) {
        let acceleration = force / self.mass_kg;
        self.velocity += acceleration * delta_time;
    }
}
