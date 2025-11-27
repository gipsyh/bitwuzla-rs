use giputils::hash::GHashMap;
use logicrs::fol::op::{self, DynOp};
use logicrs::fol::{BvConst, OpTerm, Sort, Term, TermType};
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
    fn bitwuzla_mk_term(
        tm: *mut c_void,
        kind: u32,
        argc: u32,
        args: *const *mut c_void,
    ) -> *mut c_void;
    fn bitwuzla_assert(bitwuzla: *mut c_void, term: *mut c_void);
    fn bitwuzla_check_sat(bitwuzla: *mut c_void) -> u32;
    fn bitwuzla_push(bitwuzla: *mut c_void, nlevels: u64);
    fn bitwuzla_pop(bitwuzla: *mut c_void, nlevels: u64);
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitwuzlaOp {
    Constant,
    ConstArray,
    Value,
    Variable,
    And,
    Distinct,
    Equal,
    Iff,
    Implies,
    Not,
    Or,
    Xor,
    Ite,
    Exists,
    Forall,
    Apply,
    Lambda,
    ArraySelect,
    ArrayStore,
    BvAdd,
    BvAnd,
    BvAshr,
    BvComp,
    BvConcat,
    BvDec,
    BvInc,
    BvMul,
    BvNand,
    BvNeg,
    BvNegOverflow,
    BvNor,
    BvNot,
    BvOr,
    BvRedand,
    BvRedor,
    BvRedxor,
    BvRol,
    BvRor,
    BvSaddOverflow,
    BvSdivOverflow,
    BvSdiv,
    BvSge,
    BvSgt,
    BvShl,
    BvShr,
    BvSle,
    BvSlt,
    BvSmod,
    BvSmulOverflow,
    BvSrem,
    BvSsubOverflow,
    BvSub,
    BvUaddOverflow,
    BvUdiv,
    BvUge,
    BvUgt,
    BvUle,
    BvUlt,
    BvUmulOverflow,
    BvUrem,
    BvUsubOverflow,
    BvXnor,
    BvXor,
    BvExtract,
    BvRepeat,
    BvRoli,
    BvRori,
    BvSignExtend,
    BvZeroExtend,
}

lazy_static::lazy_static! {
    static ref OP_MAP: GHashMap<DynOp, BitwuzlaOp> = {
        let mut m = GHashMap::new();
        m.insert(DynOp::from(op::Add), BitwuzlaOp::BvAdd);
        m.insert(DynOp::from(op::Eq), BitwuzlaOp::Equal);
        m.insert(DynOp::from(op::Not), BitwuzlaOp::Not);
        m.insert(DynOp::from(op::Neq), BitwuzlaOp::Distinct);
        m
    };
}

pub struct Bitwuzla {
    tm: *mut c_void,
    op: *mut c_void,
    bitwuzla: *mut c_void,
    term_map: GHashMap<Term, *mut c_void>,
}

impl Bitwuzla {
    pub fn new() -> Self {
        let tm = unsafe { bitwuzla_term_manager_new() };
        let op = unsafe { bitwuzla_options_new() };
        let bitwuzla = unsafe { bitwuzla_new(tm, op) };
        Self {
            tm,
            op,
            bitwuzla,
            term_map: GHashMap::new(),
        }
    }

    fn convert_sort(&self, sort: Sort) -> *mut c_void {
        match sort {
            Sort::Bv(w) => unsafe { bitwuzla_mk_bv_sort(self.tm, w as u64) },
            _ => todo!("support other sorts"),
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
        // let name = op_term.op.name();
        // if name == "Slice" {
        //     let arg = self.convert_term(&op_term.terms[0]);
        //     let h = op_term.terms[1].bv_len() as u64;
        //     let l = op_term.terms[2].bv_len() as u64;
        //     return unsafe {
        //         bitwuzla_mk_term1_indexed2(self.tm, BitwuzlaOp::BvExtract as u32, arg, h, l)
        //     };
        // } else if name == "Sext" {
        //     let arg = self.convert_term(&op_term.terms[0]);
        //     let ext = op_term.terms[1].bv_len() as u64;
        //     return unsafe {
        //         bitwuzla_mk_term1_indexed1(self.tm, BitwuzlaOp::BvSignExtend as u32, arg, ext)
        //     };
        // }

        let args: Vec<*mut c_void> = op_term.terms.iter().map(|t| self.convert_term(t)).collect();
        let kind = *OP_MAP
            .get(&op_term.op)
            .unwrap_or_else(|| panic!("unsupport op {:?}", op_term.op));

        unsafe { bitwuzla_mk_term(self.tm, kind as u32, args.len() as u32, args.as_ptr()) }
    }

    pub fn assert(&mut self, t: &Term) {
        let term = self.convert_term(t);
        unsafe { bitwuzla_assert(self.bitwuzla, term) }
    }

    pub fn solve(&mut self) -> bool {
        unsafe { bitwuzla_check_sat(self.bitwuzla) == 10 }
    }

    pub fn push(&mut self, nlevels: usize) {
        unsafe { bitwuzla_push(self.bitwuzla, nlevels as _) }
    }

    pub fn pop(&mut self, nlevels: usize) {
        unsafe { bitwuzla_pop(self.bitwuzla, nlevels as _) }
    }
}

impl Drop for Bitwuzla {
    fn drop(&mut self) {
        unsafe { bitwuzla_delete(self.bitwuzla) }
        unsafe { bitwuzla_term_manager_delete(self.tm) }
        unsafe { bitwuzla_options_delete(self.op) }
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
        assert!(!bzla.solve());
    }
}
