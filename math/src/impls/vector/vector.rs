use std::ops::Add;
use crate::vector::Vector;

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut values = self.values;

        for i in 0..N {
            values[i] = values[i] + rhs.values[i];
        }

        Self { values }
    }
}
