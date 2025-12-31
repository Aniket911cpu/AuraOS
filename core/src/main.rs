// core/main.rs
mod animator;
mod window;
mod window_manager;

use window_manager::WindowManager;
use cgmath::Vector3;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Initializing AuraOS Core: Liquid Glass Physics Engine...");
    
    let mut wm = WindowManager::new();
    
    // Spawn a window
    let win_id = wm.spawn_window("Terminal".to_string());
    println!("Spawned Window ID: {}", win_id);

    // Simulate interactions
    println!("Applying 'Focus' state (Active Elevation)...");
    wm.focus_window(win_id);

    // Move window to new location
    let target_pos = Vector3::new(500.0, 300.0, 0.0);
    println!("User drags window to {:?}", target_pos);
    wm.move_window(win_id, target_pos);

    // Simulation Loop
    println!("Starting Physics Loop (60Hz)...");
    for i in 0..100 {
        wm.update();
        
        if let Some(win) = wm.windows.get(&win_id) {
            let pos = win.position_anim.current;
            // Print every 10 frames to avoid spam
            if i % 10 == 0 {
                println!(
                    "Frame {:03}: Pos=({:.1}, {:.1}) Elev={:.2} [Settled={}]", 
                    i, pos.x, pos.y, win.elevation_anim.current.x, win.position_anim.is_settled
                );
            }
            
            if win.position_anim.is_settled {
                println!("Window settled at frame {}!", i);
                break;
            }
        }
        
        thread::sleep(Duration::from_millis(16)); // ~60fps
    }
    
    println!("Simulation finished.");
}
