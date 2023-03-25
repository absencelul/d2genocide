pub fn get_screen_size_x() -> Option<u32> {
    let x: *const u32 = 0x71146C as *const u32;
    if x.is_null() {
        return None;
    }
    Some(unsafe { *x })
}

pub fn get_screen_size_y() -> Option<u32> {
    let y: *const u32 = 0x711470 as *const u32;
    if y.is_null() {
        return None;
    }
    Some(unsafe { *y })
}
