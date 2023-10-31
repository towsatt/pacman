use std::fs::File;

pub fn get_sprites(){
    
}

fn get_buffer(name: &str) -> Vec<u32> {
    let decoder = png::Decoder::new(File::open(format!("assets/{name}.png")).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).unwrap();
    println!("{:?}", info);
    let u32_buffer: Vec<u32> = buffer
        .chunks_exact(4)
        .map(|v| ((v[0] as u32) << 16 | (v[1] as u32) << 8 | v[2] as u32))
        .collect();
    u32_buffer
}