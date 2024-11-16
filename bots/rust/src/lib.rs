extern "C" {
    fn print_u32(i: u32);
}

#[no_mangle]
pub extern "C" fn init(seed: u32) {
    unsafe {
        print_u32(seed);
    }
}

#[no_mangle]
pub extern "C" fn compute_actions(
    self_x: f32,
    self_y: f32,
    self_z: f32,
    ball_x: f32,
    ball_y: f32,
    ball_z: f32,
    enemy_x: f32,
    enemy_y: f32,
    enemy_z: f32,
) -> u32 {
    // Compute vector from self to ball
    let to_ball = [ball_x - self_x, ball_y - self_y, ball_z - self_z];

    // Normalize the vector
    let magnitude = (to_ball[0].powi(2) + to_ball[1].powi(2) + to_ball[2].powi(2)).sqrt();
    let normalized_to_ball = [
        to_ball[0] / magnitude,
        to_ball[1] / magnitude,
        to_ball[2] / magnitude,
    ];

    // Convert normalized vector to [u8; 3]
    let action = [
        (normalized_to_ball[0] * 127.0 + 128.0) as u8,
        (normalized_to_ball[1] * 127.0 + 128.0) as u8,
        (normalized_to_ball[2] * 127.0 + 128.0) as u8,
    ];

    // Convert [u8; 3] to u32
    let result = (action[0] as u32) << 16 | (action[1] as u32) << 8 | (action[2] as u32);

    result
}
