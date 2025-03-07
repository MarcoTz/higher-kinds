use super::{mono::MonoType, poly::PolyType, FreeTypevars};
use crate::Var;
use std::{collections::HashSet, fmt};

#[derive(Clone, PartialEq, Eq)]
pub enum RhoType {
    Mono(MonoType),
    Arrow { from: PolyType, to: PolyType },
}

impl RhoType {
    pub fn as_arrow(&self) -> Option<(&PolyType, &PolyType)> {
        if let RhoType::Arrow { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }
}

impl FreeTypevars for RhoType {
    fn free_tyvars(&self) -> HashSet<Var> {
        match self {
            RhoType::Mono(mono) => mono.free_tyvars(),
            RhoType::Arrow { from, to } => {
                let mut vars = from.free_tyvars();
                vars.extend(to.free_tyvars());
                vars
            }
        }
    }
}

impl fmt::Display for RhoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RhoType::Mono(mono) => mono.fmt(f),
            RhoType::Arrow { from, to } => write!(f, "{from} -> {to}"),
        }
    }
}

impl From<MonoType> for RhoType {
    fn from(ty: MonoType) -> RhoType {
        RhoType::Mono(ty)
    }
}
