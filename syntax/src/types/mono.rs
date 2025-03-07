use crate::Var;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MonoType {
    Int,
    Arrow {
        from: Box<MonoType>,
        to: Box<MonoType>,
    },
    TypeVar(Var),
}

impl fmt::Display for MonoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MonoType::Int => f.write_str("Int"),
            MonoType::Arrow { from, to } => write!(f, "{from}->{to}"),
            MonoType::TypeVar(v) => write!(f, "{v}"),
        }
    }
}
