use b3_math::number::{Expression, Integer};
use crate::mode::RenderMode;

pub trait Renderer<T: Integer> {
    fn debug(&self, expr: &Expression<T>) -> String;
    fn pretty(&self, expr: &Expression<T>) -> String;
    
    fn render(&self, mode: RenderMode, expr: &Expression<T>) -> String {
        match mode {
            RenderMode::Debug => self.debug(expr),
            RenderMode::Pretty => self.pretty(expr),
        }
    }
}
