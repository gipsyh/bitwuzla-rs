use std::ffi::c_void;

unsafe extern "C" {
    fn bitwuzla_term_manager_new() -> *mut c_void;
    fn bitwuzla_term_manager_delete(tm: *mut c_void);
    fn bitwuzla_options_new() -> *mut c_void;
    fn bitwuzla_options_delete(op: *mut c_void);
    fn bitwuzla_new(pm: *mut c_void, op: *mut c_void) -> *mut c_void;
    fn bitwuzla_delete(b: *mut c_void);
}

pub struct Bitwuzla {
    tm: *mut c_void,
    op: *mut c_void,
    bitwuzla: *mut c_void,
}

impl Bitwuzla {
    pub fn new() -> Self {
        let tm = unsafe { bitwuzla_term_manager_new() };
        let op = unsafe { bitwuzla_options_new() };
        let bitwuzla = unsafe { bitwuzla_new(tm, op) };
        Self { tm, op, bitwuzla }
    }
}

impl Drop for Bitwuzla {
    fn drop(&mut self) {
        unsafe { bitwuzla_delete(self.bitwuzla) }
        unsafe { bitwuzla_term_manager_delete(self.tm) }
        unsafe { bitwuzla_options_delete(self.op) }
    }
}
