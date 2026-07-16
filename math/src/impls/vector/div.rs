use crate::algebra::Div;
use crate::vector::Vector;

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut values = self.values;

        for i in 0..N {
            values[i] = values[i] / rhs;
        }

        Self { values }
    }
}
