use giputils::hash::GHashMap;
use logicrs::fol::op::{self, DynOp};

#[repr(u32)]
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BitwuzlaOp {
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
    pub(crate) static ref OP_MAP: GHashMap<DynOp, BitwuzlaOp> = {
        let mut m = GHashMap::new();
        m.insert(DynOp::from(op::Add), BitwuzlaOp::BvAdd);
        m.insert(DynOp::from(op::Eq), BitwuzlaOp::Equal);
        m.insert(DynOp::from(op::Not), BitwuzlaOp::Not);
        m.insert(DynOp::from(op::Neq), BitwuzlaOp::Distinct);
        m
    };
}
