use std::fs::File;

use elements::get_sprites;
use minifb::{ScaleMode, Window, WindowOptions, Key};

mod elements;

fn main() {
    let mut buffer = [0; 640*640];
    let mut sprites = get_sprites();
    let mut window = Window::new(
        "Test - ESC to exit",
        640,
        640,
        WindowOptions::default(),
    )
    .unwrap();
// Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..16{
            for j in 0..16{
                buffer[j*640 + i] = sprites["pacman_right_1"][j*16+i];
            }
        }
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
        .update_with_buffer(&buffer, 640, 640)
            .unwrap();
    }
}
