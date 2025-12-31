// core/window.rs
// Part of the "Liquid Glass" UI System

use cgmath::{Vector3, Quaternion, InnerSpace};
use std::time::Instant;
use crate::animator::AnimationState;

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

/// Defines the semantic role of the window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    Standard,        // Normal application window
    Installer,       // Fullscreen, high focus OOBE
    DesktopSurface,  // The background "floor"
    ContextOrb,      // The floating cognitive interface
    Modal,           // Control centers, dialogs
}

/// The fundamental UI unit in Aura OS.
/// Not just a rect, but a physical object in 3D space.
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u64,
    pub title: String,
    pub window_type: WindowType,
    
    // Spatial State (Physics Driven)
    pub position_anim: AnimationState,
    pub elevation_anim: AnimationState, 
    
    pub rotation: Quaternion<f32>,
    
    // Physical attributes
    pub mass_kg: f32,
    pub dimensions: Vector3<f32>, // Width, Height, Thickness
    pub material: MaterialProperties,
    
    // State flags
    pub is_docked: bool,
    pub last_interaction: Instant,
}

impl Window {
    pub fn new(id: u64, title: String, w_type: WindowType) -> Self {
        // Default start at 0,0,0
        let mut start_pos = Vector3::new(0.0, 0.0, 0.0);
        let mut start_elev = Vector3::new(1.0, 0.0, 0.0);
        let mut dimensions = Vector3::new(800.0, 600.0, 0.05);

        // Customize based on type
        match w_type {
            WindowType::Installer => {
                dimensions = Vector3::new(1920.0, 1080.0, 0.1); // Fullscreen, thick glass
                start_elev = Vector3::new(8.0, 0.0, 0.0);      // High focus
            },
            WindowType::DesktopSurface => {
                dimensions = Vector3::new(1920.0, 1080.0, 0.0); 
                start_elev = Vector3::new(0.0, 0.0, 0.0);      // Floor
            },
            WindowType::ContextOrb => {
                 dimensions = Vector3::new(100.0, 100.0, 0.1); // Small sphere/orb
                 start_pos = Vector3::new(0.0, -400.0, 0.0);   // Bottom center
                 start_elev = Vector3::new(2.0, 0.0, 0.0);
            },
            _ => {}
        }

        Self {
            id,
            title,
            window_type: w_type,
            position_anim: AnimationState::new(start_pos),
            elevation_anim: AnimationState::new(start_elev),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            
            mass_kg: 0.5,   // Standard window weight
            dimensions,
            material: MaterialProperties {
                refraction_index: 1.52,
                density: 1.0,
                roughness: 0.1,
            },
            is_docked: false,
            last_interaction: Instant::now(),
        }
    }

    pub fn set_elevation(&mut self, target_z: f32) {
        // We store scalar elevation in the X component of the vector for the animator
        self.elevation_anim.set_target(Vector3::new(target_z, 0.0, 0.0));
    }

    pub fn move_to(&mut self, target: Vector3<f32>) {
        self.position_anim.set_target(target);
    }

    pub fn update_physics(&mut self) {
        self.position_anim.step();
        self.elevation_anim.step();
    }

    /// Calculate the blurring radius required for the background
    /// based on the window's Z-elevation and thickness.
    pub fn calculate_blur_radius(&self) -> f32 {
        // Higher elevation = further from background = more blur
        // Thicker glass = more internal scattering
        let elev = self.elevation_anim.current.x;
        let base_blur = elev * 2.5;
        let thickness_factor = self.dimensions.z * 10.0;
        
        base_blur + thickness_factor
    }
}
