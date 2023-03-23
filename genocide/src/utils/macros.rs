#[macro_export]
macro_rules! to_cstr {
    ($rstring:expr) => {
        alloc::format!("{}\0", $rstring).as_ptr() as *const i8
    };
    ($rstring:literal) => {
        concat!($rstring, "\0").as_ptr() as *const i8
    };
}
