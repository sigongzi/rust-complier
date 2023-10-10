
use koopa::ir::BinaryOp;

use crate::ast::*;
pub trait SelectBinaryOp {
    fn select_binary_op(&self) -> BinaryOp;
}

impl SelectBinaryOp for MulOp {
    fn select_binary_op(&self) -> BinaryOp {
        match self {
            MulOp::Mul => BinaryOp::Mul,
            MulOp::Div => BinaryOp::Div,
            MulOp::Mod => BinaryOp::Mod
        }
    }
}

impl SelectBinaryOp for AddOp {
    fn select_binary_op(&self) -> BinaryOp {
        match self {
            AddOp::Add => BinaryOp::Add,
            AddOp::Sub => BinaryOp::Sub
        }
    }
}

impl SelectBinaryOp for RelOp {
    fn select_binary_op(&self) -> BinaryOp {
        match self {
            RelOp::Lt => BinaryOp::Lt,
            RelOp::Gt => BinaryOp::Gt,
            RelOp::Le => BinaryOp::Le,
            RelOp::Ge => BinaryOp::Ge
        }
    }
}

impl SelectBinaryOp for EqOp {
    fn select_binary_op(&self) -> BinaryOp {
        match self {
            EqOp::Eq => BinaryOp::Eq,
            EqOp::NotEq => BinaryOp::NotEq
        }
    }
}