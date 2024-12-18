use std::ffi::c_void;

unsafe extern "C" {
    fn bitwuzla_term_manager_new() -> *mut c_void;
}

pub struct Bitwuzla {}

impl Bitwuzla {
    pub fn new() -> Self {
        let tm = unsafe { bitwuzla_term_manager_new() };
        dbg!(tm);
        todo!();
    }
}
