use std::{collections::HashMap, fs::File};

use png::Decoder;

pub fn get_sprites() -> HashMap<String, Vec<u32>> {
    let mut hash: HashMap<String, Vec<u32>> = HashMap::new();
    let decoder = Decoder::new(File::open("sprites.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).unwrap();
    let bytes = &buffer[..info.buffer_size()];
    let vector: Vec<u32> = bytes
        .chunks(4)
        .map(|v| (v[0] as u32) << 16 | (v[1] as u32) << 8 | (v[2] as u32))
        .collect();

    let mut helper = |name, x, y| hash.insert(name, crop(&vector, (x, y)));
    for direction in 0..4 {
        for x in 0..2 {
            helper(
                format!(
                    "pacman_{:?}_{:?}",
                    ["right", "left", "up", "down"][direction],
                    x + 1
                ),
                x,
                direction,
            );
        }
    }
    hash
}

pub fn crop(image: &[u32], sprite: (usize, usize)) -> Vec<u32> {
    let (width, height) = (16, 16);
    let (x, y) = sprite;

    let mut dest = vec![];
    for i in 0..height {
        let start = (i + y) * height + (x * width);
        let end = (i + y) * height + ((x + 1) * width);
        dest.extend_from_slice(&image[start..end]);
    }

    dest
}
