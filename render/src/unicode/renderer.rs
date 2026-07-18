use b3_math::number::{Expression, Integer};
use crate::{RenderStyle, Renderer};

#[derive(Debug, Clone)]
pub struct UnicodeRenderer {
    style: RenderStyle,
}

impl UnicodeRenderer {
    pub fn new(style: RenderStyle) -> Self {
        Self { style }
    }
    
    fn visit<T>(&self, expr: &Expression<T>) -> String
    where
        T: Integer
    {
        match expr {
            Expression::Constant(_) => todo!(),
            Expression::Unary { .. } => todo!(),
            Expression::Binary { .. } => todo!(),
        }
    }
}

impl<T> Renderer<T> for UnicodeRenderer
    where T: Integer
{
    fn render(&self, expr: &Expression<T>) -> String { todo!() }
}
