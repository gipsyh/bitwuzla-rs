mod ops;
mod option;

use giputils::hash::GHashMap;
use logicrs::fol::{BvConst, OpTerm, Sort, Term, TermType, op};
use std::ffi::{CString, c_void};

unsafe extern "C" {
    fn bitwuzla_term_manager_new() -> *mut c_void;
    fn bitwuzla_term_manager_delete(tm: *mut c_void);
    fn bitwuzla_options_new() -> *mut c_void;
    fn bitwuzla_options_delete(op: *mut c_void);
    fn bitwuzla_new(pm: *mut c_void, op: *mut c_void) -> *mut c_void;
    fn bitwuzla_delete(b: *mut c_void);
    fn bitwuzla_mk_bv_sort(tm: *mut c_void, size: u64) -> *mut c_void;
    fn bitwuzla_mk_const(tm: *mut c_void, sort: *mut c_void, symbol: *const i8) -> *mut c_void;
    fn bitwuzla_mk_bv_value(
        tm: *mut c_void,
        sort: *mut c_void,
        value: *const i8,
        base: u8,
    ) -> *mut c_void;
    fn bitwuzla_mk_bv_value_uint64(tm: *mut c_void, sort: *mut c_void, value: u64) -> *mut c_void;
    fn bitwuzla_mk_term(
        tm: *mut c_void,
        kind: u32,
        argc: u32,
        args: *const *mut c_void,
    ) -> *mut c_void;
    fn bitwuzla_assert(bitwuzla: *mut c_void, term: *mut c_void);
    fn bitwuzla_check_sat(bitwuzla: *mut c_void) -> u32;
    fn bitwuzla_check_sat_assuming(
        bitwuzla: *mut c_void,
        argc: u32,
        args: *const *mut c_void,
    ) -> u32;
    fn bitwuzla_push(bitwuzla: *mut c_void, nlevels: u64);
    fn bitwuzla_pop(bitwuzla: *mut c_void, nlevels: u64);
    fn bitwuzla_get_value(bitwuzla: *mut c_void, term: *mut c_void) -> *mut c_void;
    fn bitwuzla_term_value_get_str(term: *mut c_void) -> *const i8;
    fn bitwuzla_term_is_value(term: *mut c_void) -> bool;
    fn bitwuzla_set_option(op: *mut c_void, option: u32, val: u64);
}

pub struct Bitwuzla {
    tm: *mut c_void,
    op: *mut c_void,
    bitwuzla: *mut c_void,
    term_map: GHashMap<Term, *mut c_void>,
    bv1_one: *mut c_void,
    bv1_zero: *mut c_void,
    bv2bool: GHashMap<*mut c_void, *mut c_void>,
    bool2bv: GHashMap<*mut c_void, *mut c_void>,
}

impl Bitwuzla {
    #[inline]
    fn bv1_to_bool(&mut self, bv1: *mut c_void) -> *mut c_void {
        if let Some(r) = self.bv2bool.get(&bv1) {
            return *r;
        }
        let r = unsafe {
            bitwuzla_mk_term(
                self.tm,
                ops::BitwuzlaOp::Equal as u32,
                2,
                [bv1, self.bv1_one].as_ptr(),
            )
        };
        self.bv2bool.insert(bv1, r);
        self.bool2bv.insert(r, bv1);
        r
    }

    fn bool_to_bv1(&mut self, b: *mut c_void) -> *mut c_void {
        if let Some(r) = self.bool2bv.get(&b) {
            return *r;
        }
        let r = unsafe {
            bitwuzla_mk_term(
                self.tm,
                ops::BitwuzlaOp::Ite as u32,
                3,
                [b, self.bv1_one, self.bv1_zero].as_ptr(),
            )
        };
        self.bv2bool.insert(r, b);
        self.bool2bv.insert(b, r);
        r
    }

    fn convert_sort(&self, sort: Sort) -> *mut c_void {
        match sort {
            Sort::Bv(w) => unsafe { bitwuzla_mk_bv_sort(self.tm, w as u64) },
            _ => todo!("unsupport other sorts"),
        }
    }

    fn convert_term(&mut self, term: &Term) -> *mut c_void {
        if let Some(&t) = self.term_map.get(term) {
            return t;
        }

        let res = match &**term {
            TermType::Const(c) => self.convert_const(c, term.sort()),
            TermType::Var(id) => self.convert_var(*id, term.sort()),
            TermType::Op(op) => self.convert_op(op),
        };

        self.term_map.insert(term.clone(), res);
        res
    }

    fn convert_const(&self, c: &BvConst, sort: Sort) -> *mut c_void {
        let sort_ptr = self.convert_sort(sort);
        let mut s = String::new();
        for b in c.iter().rev() {
            s.push(if *b { '1' } else { '0' });
        }
        let c_str = CString::new(s).unwrap();
        unsafe { bitwuzla_mk_bv_value(self.tm, sort_ptr, c_str.as_ptr(), 2) }
    }

    fn convert_var(&self, _id: usize, sort: Sort) -> *mut c_void {
        let sort_ptr = self.convert_sort(sort);
        unsafe { bitwuzla_mk_const(self.tm, sort_ptr, std::ptr::null()) }
    }

    fn convert_op(&mut self, op_term: &OpTerm) -> *mut c_void {
        let mut args: Vec<*mut c_void> =
            op_term.terms.iter().map(|t| self.convert_term(t)).collect();

        if op_term.op == op::Ite {
            args[0] = self.bv1_to_bool(args[0]);
        }
        if op_term.op == op::Implies {
            args[0] = self.bv1_to_bool(args[0]);
            args[1] = self.bv1_to_bool(args[1]);
        }

        let kind = *ops::OP_MAP
            .get(&op_term.op)
            .unwrap_or_else(|| panic!("unsupport op {:?}", op_term.op));

        let res =
            unsafe { bitwuzla_mk_term(self.tm, kind as u32, args.len() as u32, args.as_ptr()) };

        match kind {
            ops::BitwuzlaOp::Equal
            | ops::BitwuzlaOp::Distinct
            | ops::BitwuzlaOp::BvUlt
            | ops::BitwuzlaOp::BvUgt
            | ops::BitwuzlaOp::BvSlt
            | ops::BitwuzlaOp::BvSgt => self.bool_to_bv1(res),
            _ => res,
        }
    }
}

impl Bitwuzla {
    pub fn new() -> Self {
        let tm = unsafe { bitwuzla_term_manager_new() };
        let op = unsafe { bitwuzla_options_new() };
        unsafe { bitwuzla_set_option(op, option::BitwuzlaOption::ProduceModels as u32, 1) };
        let bitwuzla = unsafe { bitwuzla_new(tm, op) };

        let bv1_sort = unsafe { bitwuzla_mk_bv_sort(tm, 1) };
        let bv1_one = unsafe { bitwuzla_mk_bv_value_uint64(tm, bv1_sort, 1) };
        let bv1_zero = unsafe { bitwuzla_mk_bv_value_uint64(tm, bv1_sort, 0) };

        Self {
            tm,
            op,
            bitwuzla,
            term_map: GHashMap::new(),
            bv1_one,
            bv1_zero,
            bv2bool: GHashMap::new(),
            bool2bv: GHashMap::new(),
        }
    }

    pub fn assert(&mut self, t: &Term) {
        let term = self.convert_term(t);
        let term = self.bv1_to_bool(term);
        unsafe { bitwuzla_assert(self.bitwuzla, term) }
    }

    pub fn solve<'a>(&mut self, assumps: impl IntoIterator<Item = &'a Term>) -> bool {
        let assumps: Vec<*mut c_void> = assumps
            .into_iter()
            .map(|t| {
                debug_assert!(t.is_bool());
                let term = self.convert_term(t);
                self.bv1_to_bool(term)
            })
            .collect();
        let res = if assumps.is_empty() {
            unsafe { bitwuzla_check_sat(self.bitwuzla) }
        } else {
            unsafe {
                bitwuzla_check_sat_assuming(self.bitwuzla, assumps.len() as u32, assumps.as_ptr())
            }
        };
        res == 10
    }

    pub fn push(&mut self, nlevels: usize) {
        unsafe { bitwuzla_push(self.bitwuzla, nlevels as _) }
    }

    pub fn pop(&mut self, nlevels: usize) {
        unsafe { bitwuzla_pop(self.bitwuzla, nlevels as _) }
    }

    pub fn sat_value(&mut self, term: &Term) -> BvConst {
        let t = self.convert_term(term);
        let val = unsafe { bitwuzla_get_value(self.bitwuzla, t) };
        debug_assert!(unsafe { bitwuzla_term_is_value(val) });
        let s_ptr = unsafe { bitwuzla_term_value_get_str(val) };
        let s = unsafe { std::ffi::CStr::from_ptr(s_ptr).to_string_lossy() };
        let bits: Vec<bool> = s.chars().rev().map(|c| c == '1').collect();
        BvConst::new(&bits)
    }
}

impl Drop for Bitwuzla {
    fn drop(&mut self) {
        unsafe { bitwuzla_delete(self.bitwuzla) }
        unsafe { bitwuzla_term_manager_delete(self.tm) }
        unsafe { bitwuzla_options_delete(self.op) }
    }
}

impl Default for Bitwuzla {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logicrs::fol::{
        Sort, Term,
        op::{self},
    };

    #[test]
    fn test0() {
        let mut bzla = Bitwuzla::new();
        let a = Term::new_var(Sort::Bv(2));
        let b = Term::new_var(Sort::Bv(2));
        let a_add_b = a.op1(op::Add, &b);
        let b_add_a = b.op1(op::Add, &a);
        bzla.assert(&a_add_b.op1(op::Neq, &b_add_a));
        assert!(!bzla.solve([]));
    }

    #[test]
    fn test1() {
        let mut bzla = Bitwuzla::new();
        let bv2c2 = BvConst::from_usize(2, 2);
        let t_bv2c2 = Term::bv_const(bv2c2.clone());
        let a = Term::new_var(Sort::Bv(2));
        let b = Term::new_var(Sort::Bv(2));
        let a_eq_2 = a.op1(op::Eq, &t_bv2c2);
        let a_eq_b = a.op1(op::Eq, &b);
        assert!(bzla.solve(&[a_eq_2, a_eq_b]));
        assert!(bzla.sat_value(&a).eq(&bv2c2));
        assert!(bzla.sat_value(&b).eq(&bv2c2));
    }

    #[test]
    fn test2() {
        let mut bzla = Bitwuzla::new();
        let bv1c1 = BvConst::from_usize(1, 1);
        let t_bv1c1 = Term::bv_const(bv1c1);
        bzla.assert(&t_bv1c1);
    }

    #[test]
    fn concat() {
        let mut bzla = Bitwuzla::new();
        let bv2c3 = BvConst::from_usize(3, 2);
        let t_bv2c3 = Term::bv_const(bv2c3);
        let bv3c0 = BvConst::from_usize(0, 3);
        let t_bv3c0 = Term::bv_const(bv3c0);
        let bv5c3 = BvConst::from_usize(3, 5);
        let t_bv5c3 = Term::bv_const(bv5c3);
        let tneq = t_bv5c3.tneq(&t_bv3c0.concat(&t_bv2c3));
        assert!(!bzla.solve(&[tneq]));
    }
}
