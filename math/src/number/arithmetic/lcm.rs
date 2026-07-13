pub trait LeastCommonMultiple<Rhs = Self> {
    fn lcm(&self, rhs: &Rhs) -> Self;
}
