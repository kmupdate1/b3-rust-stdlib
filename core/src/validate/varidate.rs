pub trait Varidate {
    type Error;
    
    fn validate(&self) -> Result<(), Self::Error>;
}
