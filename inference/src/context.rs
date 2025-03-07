use super::errors::Error;
use syntax::{types::PolyType, Var};

#[derive(Clone)]
pub struct VarBinding {
    var: Var,
    ty: PolyType,
}

#[derive(Clone)]
pub struct Context {
    vars: Vec<VarBinding>,
}

impl Context {
    pub fn find_var(&self, v: &Var) -> Result<PolyType, Error> {
        self.vars
            .iter()
            .find(|bnd| bnd.var == *v)
            .map(|bnd| bnd.ty.clone())
            .ok_or(Error::UnboundVariable { var: v.to_owned() })
    }

    pub fn add_var(&mut self, v: &Var, ty: &PolyType) {
        self.vars.push(VarBinding {
            var: v.to_owned(),
            ty: ty.clone(),
        })
    }
}
