use super::errors::Error;
use syntax::types::{PolyType, RhoType};

pub fn dsk(_lower: &PolyType, _upper: &PolyType) -> Result<(), Error> {
    todo!()
}

pub fn dsk_star(_lower: &PolyType, _upper: &RhoType) -> Result<(), Error> {
    todo!()
}

pub fn pr(_ty: &PolyType) -> PolyType {
    todo!()
}
