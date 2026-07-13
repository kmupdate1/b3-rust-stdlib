pub trait Divisors {
    type DivisorSet;
    fn divisors(&self) -> Self::DivisorSet;
}

pub struct DivisorSet<T> {
    divisors: Vec<T>,
}

impl<T> DivisorSet<T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.divisors.iter()
    }

    pub fn len(&self) -> usize { self.divisors.len() }
}
