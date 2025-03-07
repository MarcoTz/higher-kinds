use super::{context::Context, errors::Error};
use syntax::{types::PolyType, Term};

pub fn check_poly(_t: &Term, _ty: &PolyType, _ctx: &mut Context) -> Result<(), Error> {
    todo!()
}

pub fn infer_poly(_t: &Term, _ctx: &mut Context) -> Result<PolyType, Error> {
    todo!()
}
