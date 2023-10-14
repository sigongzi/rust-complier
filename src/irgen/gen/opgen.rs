
use koopa::ir::{BinaryOp};

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

/* 
pub fn calculate_in_advance(op : BinaryOp, l : &ValueData, r : &ValueData) -> Result<i32>{

    let (lhs, rhs) = match (l.kind(),r.kind()) {
        (ValueKind::Integer(l), ValueKind::Integer(r)) => 
        (l.value(), r.value()),
        _ => {
            return Err(IRError::AdvancedEvaluation("value is not integer".to_string()));
        }
    };

    match op{
        BinaryOp::Mul => Ok(lhs * rhs),
        BinaryOp::Div => Ok(lhs / rhs),
        BinaryOp::Mod => Ok(lhs % rhs),
        BinaryOp::Sub => Ok(lhs - rhs),
        BinaryOp::Add => Ok(lhs + rhs),
        BinaryOp::Lt => Ok((lhs < rhs) as i32),
        BinaryOp::Gt => Ok((lhs > rhs) as i32),
        BinaryOp::Le => Ok((lhs <= rhs) as i32),
        BinaryOp::Ge => Ok((lhs >= rhs) as i32),
        BinaryOp::Eq => Ok((lhs == rhs) as i32),
        BinaryOp::NotEq => Ok((lhs != rhs) as i32),
        _ => Err(IRError::AdvancedEvaluation("Unsupported binary operator".to_string()))
    }
}*/