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
        m.insert(DynOp::from(op::Sub), BitwuzlaOp::BvSub);
        m.insert(DynOp::from(op::Mul), BitwuzlaOp::BvMul);
        m.insert(DynOp::from(op::Udiv), BitwuzlaOp::BvUdiv);
        m.insert(DynOp::from(op::Sdiv), BitwuzlaOp::BvSdiv);
        m.insert(DynOp::from(op::Urem), BitwuzlaOp::BvUrem);
        m.insert(DynOp::from(op::Srem), BitwuzlaOp::BvSrem);
        m.insert(DynOp::from(op::Eq), BitwuzlaOp::Equal);
        m.insert(DynOp::from(op::Neq), BitwuzlaOp::Distinct);
        m.insert(DynOp::from(op::Not), BitwuzlaOp::Not);
        m.insert(DynOp::from(op::And), BitwuzlaOp::And);
        m.insert(DynOp::from(op::Or), BitwuzlaOp::Or);
        m.insert(DynOp::from(op::Xor), BitwuzlaOp::Xor);
        m.insert(DynOp::from(op::Implies), BitwuzlaOp::Implies);
        m.insert(DynOp::from(op::Ite), BitwuzlaOp::Ite);
        m.insert(DynOp::from(op::Ult), BitwuzlaOp::BvUlt);
        m.insert(DynOp::from(op::Ugt), BitwuzlaOp::BvUgt);
        m.insert(DynOp::from(op::Slt), BitwuzlaOp::BvSlt);
        m.insert(DynOp::from(op::Sgt), BitwuzlaOp::BvSgt);

        // m.insert(DynOp::from(op::Concat), BitwuzlaOp::BvConcat);

        m
    };
}
