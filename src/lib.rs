mod ops;
mod option;

#[cfg(bitwuzla_stub)]
mod stub;
#[cfg(bitwuzla_stub)]
pub use stub::Bitwuzla;

#[cfg(not(bitwuzla_stub))]
mod ffi;
#[cfg(not(bitwuzla_stub))]
pub use ffi::Bitwuzla;

#[cfg(test)]
mod tests {
    use super::*;
    use giputils::bitvec::BitVec;
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
        let bv2c2 = BitVec::from_usize(2, 2);
        let t_bv2c2 = Term::bv_const(bv2c2.clone());
        let a = Term::new_var(Sort::Bv(2));
        let b = Term::new_var(Sort::Bv(2));
        let a_eq_2 = a.op1(op::Eq, &t_bv2c2);
        let a_eq_b = a.op1(op::Eq, &b);
        assert!(bzla.solve(&[a_eq_2, a_eq_b]));
        assert!(bzla.sat_value(&a).unwrap().eq(&bv2c2));
        assert!(bzla.sat_value(&b).unwrap().eq(&bv2c2));
    }

    #[test]
    fn test2() {
        let mut bzla = Bitwuzla::new();
        let bv1c1 = BitVec::from_usize(1, 1);
        let t_bv1c1 = Term::bv_const(bv1c1);
        bzla.assert(&t_bv1c1);
    }

    #[test]
    fn concat() {
        let mut bzla = Bitwuzla::new();
        let bv2c3 = BitVec::from_usize(2, 3);
        let t_bv2c3 = Term::bv_const(bv2c3);
        let bv3c0 = BitVec::from_usize(3, 0);
        let t_bv3c0 = Term::bv_const(bv3c0);
        let bv5c3 = BitVec::from_usize(5, 3);
        let t_bv5c3 = Term::bv_const(bv5c3);
        let tneq = t_bv5c3.tneq(&t_bv3c0.concat(&t_bv2c3));
        assert!(!bzla.solve(&[tneq]));
    }

    #[test]
    fn slice() {
        let mut bzla = Bitwuzla::new();
        let a = Term::new_var(Sort::Bv(4));
        let slice = a.slice(1, 3);
        let c14 = Term::bv_const(BitVec::from_usize(4, 14));
        bzla.assert(&a.teq(&c14));
        assert!(bzla.solve([]));
        let val = bzla.sat_value(&slice);
        assert_eq!(val.unwrap(), BitVec::from_usize(3, 7));
    }
}
