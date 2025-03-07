use super::{
    context::Context,
    dsk::dsk,
    errors::Error,
    inst::check_inst,
    poly::{check_poly, infer_poly},
    typeinfer::infer_type,
};
use syntax::{
    types::{MonoType, RhoType},
    Term,
};

pub fn check_type(t: &Term, ty: &RhoType, ctx: &mut Context) -> Result<(), Error> {
    match t {
        Term::Literal(_) => {
            if *ty == MonoType::Int.into() {
                Ok(())
            } else {
                Err(Error::UnexpectedRhoType {
                    found: ty.clone(),
                    expected: MonoType::Int.into(),
                })
            }
        }
        Term::Variable(var) => {
            let ctx_ty = ctx.find_var(var)?;
            check_inst(&ctx_ty, ty)
        }
        Term::Abs { var, t } => {
            let (poly_from, poly_to) = ty.as_arrow().ok_or(Error::NoArrow { ty: ty.clone() })?;
            ctx.add_var(var, poly_from);
            check_poly(t, poly_to, ctx)
        }
        Term::TyAbs { var, ty: annot, t } => {
            let (poly_from, poly_to) = ty.as_arrow().ok_or(Error::NoArrow { ty: ty.clone() })?;
            dsk(poly_from, annot)?;
            ctx.add_var(var, annot);
            check_poly(t, poly_to, ctx)
        }
        Term::App { fun, arg } => {
            let fun_ty = infer_type(fun, &mut ctx.clone())?;
            let (poly_from, poly_to) = fun_ty
                .as_arrow()
                .ok_or(Error::NoArrow { ty: fun_ty.clone() })?;
            check_poly(arg, poly_from, ctx)?;
            check_inst(poly_to, ty)
        }
        Term::Annot { t, ty: annot } => {
            check_poly(t, annot, ctx)?;
            check_inst(annot, ty)
        }
        Term::Let {
            var,
            bound_term,
            in_term,
        } => {
            let bound_ty = infer_poly(bound_term, ctx)?;
            ctx.add_var(var, &bound_ty);
            check_type(in_term, ty, ctx)
        }
    }
}
