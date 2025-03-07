use std::fmt;
use syntax::{RhoType, Var};

#[derive(Debug)]
pub enum Error {
    UnexpectedRhoType { found: RhoType, expected: RhoType },
    UnboundVariable { var: Var },
    NoArrow { ty: RhoType },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedRhoType { found, expected } => {
                write!(f, "Unexpected rho type {found}, expected {expected}")
            }
            Error::UnboundVariable { var } => write!(f, "Unbound Variable {var}"),
            Error::NoArrow { ty } => write!(f, "{ty} is not an arrow type"),
        }
    }
}

impl std::error::Error for Error {}
