use super::{mono::MonoType, poly::PolyType};
use std::fmt;

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
