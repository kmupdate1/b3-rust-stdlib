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

impl<T> DivisorSet<T> {
    pub fn new(divisors: Vec<T>) -> Self {
        Self { divisors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisor_set_iteration() {
        let set = DivisorSet {
            divisors: vec![1, 2, 3, 6],
        };

        assert_eq!(set.len(), 4);

        let values: Vec<_> = set.iter().copied().collect();

        assert_eq!(values, vec![1, 2, 3, 6]);
    }
}
