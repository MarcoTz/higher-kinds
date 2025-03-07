use super::{
    context::Context, dsk::pr, errors::Error, typecheck::check_type, typeinfer::infer_type,
};
use syntax::{
    types::{FreeTypevars, PolyType},
    Term,
};

pub fn check_poly(t: &Term, ty: &PolyType, ctx: &mut Context) -> Result<(), Error> {
    let ty_poly = pr(ty);
    let mut vars = ctx.free_tyvars();
    vars.retain(|v| ty_poly.vars.contains(&v));
    if !vars.is_empty() {
        return Err(Error::BoundMultipleTimes {
            var: vars.into_iter().next().unwrap(),
        });
    }
    check_type(t, &ty_poly.ty, ctx)
}

pub fn infer_poly(t: &Term, ctx: &mut Context) -> Result<PolyType, Error> {
    let mut vars = ctx.free_tyvars();
    let rho_ty = infer_type(t, ctx)?;
    vars.extend(rho_ty.free_tyvars());
    Ok(PolyType {
        vars: vars.into_iter().collect(),
        ty: Box::new(rho_ty),
    })
}
