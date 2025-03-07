use super::{FreeTypevars, SubstTypevar};
use crate::Var;
use std::{collections::HashSet, fmt};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MonoType {
    Int,
    Arrow {
        from: Box<MonoType>,
        to: Box<MonoType>,
    },
    TypeVar(Var),
}

impl FreeTypevars for MonoType {
    fn free_tyvars(&self) -> HashSet<Var> {
        match self {
            MonoType::Int => HashSet::new(),
            MonoType::Arrow { from, to } => {
                let mut vars = from.free_tyvars();
                vars.extend(to.free_tyvars());
                vars
            }
            MonoType::TypeVar(var) => HashSet::from([var.clone()]),
        }
    }
}

impl SubstTypevar for MonoType {
    fn subst_tyvar(self, v: Var, ty: MonoType) -> Self {
        match self {
            MonoType::Int => MonoType::Int,
            MonoType::Arrow { from, to } => MonoType::Arrow {
                from: Box::new(from.subst_tyvar(v.clone(), ty.clone())),
                to: Box::new(to.subst_tyvar(v, ty)),
            },
            MonoType::TypeVar(var) => {
                if var == v {
                    ty
                } else {
                    MonoType::TypeVar(var)
                }
            }
        }
    }
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
