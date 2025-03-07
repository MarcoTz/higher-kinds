use super::{dsk::dsk, errors::Error};
use syntax::{
    types::{MonoType, PolyType, RhoType, SubstTypevar},
    Var,
};

pub fn check_inst(lower: &PolyType, upper: &RhoType) -> Result<(), Error> {
    dsk(lower, &upper.clone().into())
}

pub fn infer_inst(lower: &PolyType) -> Result<RhoType, Error> {
    //these need to be reconstructed
    let monos: Vec<(Var, MonoType)> = lower
        .vars
        .iter()
        .map(|v| (v.clone(), MonoType::Int))
        .collect();
    let mut rho = *lower.ty.clone();
    for (v, ty) in monos {
        rho = rho.subst_tyvar(v, ty);
    }
    Ok(rho)
}
