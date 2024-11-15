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
pub extern "C" fn update(_data1: u32, _data2: u32) -> u32 {
    return 5;
}
