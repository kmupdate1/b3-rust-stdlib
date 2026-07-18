use b3_math::number::{Expression, Integer};

pub trait Renderer<T: Integer> {
    fn render(&self, expr: &Expression<T>) -> String;
}
