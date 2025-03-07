use super::Var;
use std::collections::HashSet;

mod mono;
mod poly;
mod rho;

pub use mono::MonoType;
pub use poly::PolyType;
pub use rho::RhoType;

pub trait FreeTypevars {
    fn free_tyvars(&self) -> HashSet<Var>;
}

pub trait SubstTypevar {
    fn subst_tyvar(self, v: Var, ty: MonoType) -> Self;
}
