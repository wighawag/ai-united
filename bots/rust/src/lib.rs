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
pub extern "C" fn update(_my_units: u32, _enemy_units: u32) -> u32 {
    return 5;
}
