use super::{mono::MonoType, rho::RhoType, FreeTypevars, SubstTypevar};
use crate::Var;
use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolyType {
    pub vars: Vec<Var>,
    pub ty: Box<RhoType>,
}

impl FreeTypevars for PolyType {
    fn free_tyvars(&self) -> HashSet<Var> {
        let mut vars = self.ty.free_tyvars();
        for v in self.vars.iter() {
            vars.remove(v);
        }
        vars
    }
}

impl SubstTypevar for PolyType {
    fn subst_tyvar(self, v: Var, ty: MonoType) -> Self {
        if self.vars.contains(&v) {
            self
        } else {
            PolyType {
                vars: self.vars,
                ty: Box::new(self.ty.subst_tyvar(v, ty)),
            }
        }
    }
}

impl From<RhoType> for PolyType {
    fn from(rho: RhoType) -> PolyType {
        PolyType {
            vars: Vec::new(),
            ty: Box::new(rho),
        }
    }
}

impl From<MonoType> for PolyType {
    fn from(mono: MonoType) -> PolyType {
        let rho: RhoType = mono.into();
        rho.into()
    }
}

impl fmt::Display for PolyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}.{}", self.vars.join(" "), self.ty)
    }
}
