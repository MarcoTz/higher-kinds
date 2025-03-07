use super::errors::Error;
use syntax::types::{PolyType, RhoType};

pub fn check_inst(_lower: &PolyType, _upper: &RhoType) -> Result<(), Error> {
    todo!()
}

pub fn infer_inst(_lower: &PolyType) -> Result<RhoType, Error> {
    todo!()
}
