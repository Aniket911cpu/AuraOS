// core/main.rs
mod animator;
mod window;
mod window_manager;

use window_manager::WindowManager;
use window::WindowType;
use cgmath::Vector3;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Initializing AuraOS Core: Liquid Glass Physics Engine...");
    
    let mut wm = WindowManager::new();
    
    println!("\n--- STEP 1: OOBE (Installer) ---");
    let installer_id = wm.spawn_window("Prism Installer".to_string(), WindowType::Installer);
    println!("Spawned Installer (ID: {}). Fullscreen, Elevated.", installer_id);
    
    // Simulate setup time
    run_sim(&mut wm, 30); 

    println!("\n--- STEP 2: Setup Complete -> Transition to Homescreen ---");
    println!("Closing Installer...");
    wm.close_window(installer_id);
    
    println!("Spawning Desktop Surface & Context Orb...");
    let desktop_id = wm.spawn_window("Ethereal Desktop".to_string(), WindowType::DesktopSurface);
    let orb_id = wm.spawn_window("Context Orb".to_string(), WindowType::ContextOrb);
    
    run_sim(&mut wm, 20);

    println!("\n--- STEP 3: User Opens App ---");
    let app_id = wm.spawn_window("Terminal".to_string(), WindowType::Standard);
    println!("Spawned Terminal (ID: {}).", app_id);
    wm.focus_window(app_id);

    println!("User drags Terminal...");
    wm.move_window(app_id, Vector3::new(200.0, 100.0, 0.0));
    
    run_sim(&mut wm, 60); // Let it settle
    
    println!("Simulation finished.");
}

fn run_sim(wm: &mut WindowManager, frames: usize) {
    for _ in 0..frames {
        wm.update();
        thread::sleep(Duration::from_millis(16));
    }
}
