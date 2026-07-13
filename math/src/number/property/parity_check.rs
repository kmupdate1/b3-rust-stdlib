use crate::number::Parity;

pub trait ParityCheck {
    fn parity(&self) -> Parity;
}
