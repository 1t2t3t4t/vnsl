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
    Arithmetic(VnslArithOps),
    Comparison(VnslCompareOps),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslArithOps {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VnslCompareOps {
    Eq,
    NEq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslRhsOp {
    pub op: VnslOps,
    pub rhs: Box<VnslExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnslOpExpr {
    pub lhs: VnslAtom,
    pub rhs_op: Option<VnslRhsOp>,
}

impl VnslOpExpr {
    pub fn is_atom(&self) -> bool {
        self.rhs_op.is_none()
    }
}
