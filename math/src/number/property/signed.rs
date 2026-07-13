use crate::number::Sign;

pub trait Signed {
    fn sign(&self) -> Sign;
}
