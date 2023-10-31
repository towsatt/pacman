use std::fs::File;

use minifb::{ScaleMode, Window, WindowOptions, Key};

fn main() {
    let decoder = png::Decoder::new(File::open("textures/assets.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buffer).unwrap();
    let u32_buffer: Vec<u32> = buffer
        .chunks_exact(4)
        .map(|v| ((v[2] as u32) << 24 | (v[1] as u32) << 16 | v[0] as u32) << 8 | v[3] as u32)
        .collect();
    let mut window = Window::new(
        "assets",
        reader.info().width as usize,
        reader.info().height as usize,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &u32_buffer,
                reader.info().width as usize,
                reader.info().height as usize,
            )
            .unwrap();
    }
}
