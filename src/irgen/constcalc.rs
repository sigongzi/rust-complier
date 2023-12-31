use crate::ast::*;

use crate::irgen::scopes::Scopes;
use crate::irgen::IResult;
use crate::irgen::IRError;


use super::scopes::RecValue;

// just calculate the result to the primitive type : i32
pub trait PreCalculate<'ast> {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32>;
}

impl<'ast> PreCalculate<'ast> for Exp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        self.lor.calculate(scope)
    }
}

impl<'ast> PreCalculate<'ast> for LOrExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::LAndAusdruck(a) => a.calculate(scope),
            Self::LOrAusdruck(lhs, rhs) => {
                Ok((lhs.calculate(scope)? != 0 || rhs.calculate(scope)? != 0) as i32)
            }
        }
    }
}

impl<'ast> PreCalculate<'ast> for LAndExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::EqAusdruck(a) => a.calculate(scope),
            Self::LAndAusdruck(lhs, rhs) => {
                Ok((lhs.calculate(scope)? != 0 && rhs.calculate(scope)? != 0) as i32)
            }
        }
    }
}

impl<'ast> PreCalculate<'ast> for EqExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::RelAusdruck(a) => a.calculate(scope),
            Self::EqAusdruck(lhs, op, rhs) => {
                let (a,b) = 
                (lhs.calculate(scope)?, rhs.calculate(scope)?);
                
                match op {
                    EqOp::Eq => Ok((a == b) as i32),
                    EqOp::NotEq => Ok((a != b) as i32)
                }
            }
        }
    }
}

impl<'ast> PreCalculate<'ast> for RelExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::AddAusdruck(a) => a.calculate(scope),
            Self::RelAusdruck(lhs, op, rhs) => {
                let (a, b) = (lhs.calculate(scope)?, rhs.calculate(scope)?);

                match op {
                    RelOp::Ge => Ok((a >= b) as i32),
                    RelOp::Gt => Ok((a > b) as i32),
                    RelOp::Le => Ok((a <= b) as i32),
                    RelOp::Lt => Ok((a < b) as i32)
                }

            }

        }
    }
}

impl<'ast> PreCalculate<'ast> for AddExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::MulAusdruck(e) => e.calculate(scope),
            Self::AddAusdruck(lhs, op, rhs) => {
                let (a,b) = (lhs.calculate(scope)?, rhs.calculate(scope)?);

                match op {
                    AddOp::Add => Ok(a + b),
                    AddOp::Sub => Ok(a - b)
                }
            }
        }
    }
}

impl<'ast> PreCalculate<'ast> for MulExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::UnaryAusdruck(e) => e.calculate(scope),
            Self::MulAusdruck(lhs, op, rhs) => {
                let (a, b) = (lhs.calculate(scope)?, rhs.calculate(scope)?);

                match op {
                    MulOp::Div => Ok(a / b),
                    MulOp::Mod => Ok(a % b),
                    MulOp::Mul => Ok(a * b)
                }
            }
        }
    }
}

impl<'ast> PreCalculate<'ast> for UnaryExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::PrimaryAusdruck(e) => e.calculate(scope),
            Self::Call(_) => Err(IRError::EvaluateConstWithCall),
            Self::UnaryAusdruck(op, num) => {
                let a = num.calculate(scope)?;
                match op {
                    UnaryOp::LNot => Ok(!(a == 0) as i32),
                    UnaryOp::Negative => Ok(-a),
                    UnaryOp::Positive => Ok(a)
                }
            }
        }        
    }
}

impl<'ast> PreCalculate<'ast> for PrimaryExp {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match self {
            Self::Ausdruck(e) => e.calculate(scope),
            Self::LVal(v) => v.calculate(scope),
            Self::Number(n) => Ok(*n)
        }
    }
}

impl<'ast> PreCalculate<'ast> for LVal {
    fn calculate(&'ast self, scope :&mut Scopes<'ast>) -> IResult<i32> {
        match scope.retrieve_val(self.id.as_str()) {
            Some(val) => {
                match val {
                    RecValue::Const(v) => Ok(v),
                    RecValue::IrValue(_) => Err(IRError::EvaluateConstWithVar)
                }
            },
            None => Err(IRError::UndefinedLVal(self.id.clone()))
            
        }
    }
}