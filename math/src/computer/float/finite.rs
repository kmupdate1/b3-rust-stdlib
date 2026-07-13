use crate::computer::Finiteness;

pub trait Finite {
    fn finiteness(&self) -> Finiteness;
}
