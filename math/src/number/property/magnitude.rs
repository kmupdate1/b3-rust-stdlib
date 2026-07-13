pub trait Magnitude {
    type Output;

    fn magnitude(&self) -> Self::Output;
}
