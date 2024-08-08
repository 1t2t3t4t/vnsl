use crate::model::VnslDataType;

#[derive(Debug, Clone, PartialEq)]
pub enum VnslAtom {
    Literal(VnslDataType),
    Group(Box<VnslExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslExpr {
    Op(VnslOpExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslOps {
    Arithmetic(),
    Comparison(),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslRhsOp {
    op: VnslOps,
    rhs: Box<VnslExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslOpExpr {
    lhs: VnslAtom,
    rhs_op: Option<VnslRhsOp>,
}
