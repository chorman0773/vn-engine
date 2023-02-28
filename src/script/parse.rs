
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum BinaryOp{
    Add,
    Sub,
    Mul,
    Div,
    BitAnd,
    BitOr,
    BitXor,
    LogicAnd,
    LogicOr,
    LeftShift,
    RightShift,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    CmpEq,
    CmpNe,
    CmpLt,
    CmpGt,
    CmpLe,
    CmpGe,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnaryOp{
    Neg,
    Not,
    
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Expr{

}