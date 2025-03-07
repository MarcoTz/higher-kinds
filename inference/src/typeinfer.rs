use super::{
    context::Context,
    errors::Error,
    inst::infer_inst,
    poly::{check_poly, infer_poly},
};
use syntax::{
    types::{MonoType, RhoType},
    Term,
};

pub fn infer_type(t: &Term, ctx: &mut Context) -> Result<RhoType, Error> {
    match t {
        Term::Literal(_) => Ok(MonoType::Int.into()),
        Term::Variable(v) => {
            let ctx_ty = ctx.find_var(v)?;
            let ty = infer_inst(&ctx_ty)?;
            Ok(ty)
        }
        Term::Abs { var, t } => {
            // This monotype needs to be reconstructed
            let from_ty = MonoType::Int.into();
            ctx.add_var(var, &from_ty);
            let to_ty = infer_type(t, ctx)?;
            Ok(RhoType::Arrow {
                from: from_ty,
                to: to_ty.into(),
            })
        }
        Term::TyAbs { var, ty, t } => {
            ctx.add_var(var, ty);
            let to_ty = infer_type(t, ctx)?;
            Ok(RhoType::Arrow {
                from: ty.clone(),
                to: to_ty.into(),
            })
        }
        Term::App { fun, arg } => {
            let from_ty = infer_type(fun, &mut ctx.clone())?;
            let (poly_from, poly_to) = from_ty.as_arrow().ok_or(Error::NoArrow {
                ty: from_ty.clone(),
            })?;
            check_poly(arg, poly_from, ctx)?;
            infer_inst(poly_to)
        }
        Term::Annot { t, ty } => {
            check_poly(t, ty, ctx)?;
            infer_inst(ty)
        }
        Term::Let {
            var,
            bound_term,
            in_term,
        } => {
            let bound_ty = infer_poly(bound_term, &mut ctx.clone())?;
            ctx.add_var(var, &bound_ty);
            infer_type(in_term, ctx)
        }
    }
}
