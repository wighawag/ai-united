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
    self_x: u32,
    self_y: u32,
    ball_x: u32,
    ball_y: u32,
    enemy_x: u32,
    enemy_y: u32,
) -> u32 {
    return 1;
}
