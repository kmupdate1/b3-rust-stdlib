use crate::{RenderStyle, Renderer};
use b3_math::number::{BinaryOperator, Constant, Expression, Integer, UnaryOperator};
use std::fmt::Display;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct UnicodeRenderer {
    style: RenderStyle,
}

impl UnicodeRenderer {
    pub fn new(style: RenderStyle) -> Self {
        Self { style }
    }

    fn visit<T>(&self, expr: &Expression<T>, out: &mut String)
    where
        T: Integer + Display,
    {
        match expr {
            Expression::Constant(constant) =>
                self.visit_constant(constant, out),
            Expression::Unary { op, operand } =>
                self.visit_unary(op, operand, out),
            Expression::Binary { op, lhs, rhs } =>
                self.visit_binary(op, lhs, rhs, out),
        }
    }

    fn visit_constant<T>(&self, constant: &Constant<T>, out: &mut String)
    where
        T: Integer + Display,
    { let _ = write!(out, "({constant})"); }

    fn visit_unary<T>(
        &self,
        op: &UnaryOperator,
        operand: &Expression<T>,
        out: &mut String,
    )
    where
        T: Integer + Display,
    {
        out.push('{');

        let _ = write!(out, "{op}");

        self.visit(operand, out);

        out.push('}');
    }

    fn visit_binary<T>(
        &self,
        op: &BinaryOperator,
        lhs: &Expression<T>,
        rhs: &Expression<T>,
        out: &mut String,
    )
    where
        T: Integer + Display,
    {
        out.push('[');

        self.visit(lhs, out);

        let _ = write!(out, " {op} ");

        self.visit(rhs, out);

        out.push(']');
    }
}

impl<T> Renderer<T> for UnicodeRenderer
    where T: Integer + Display,
{
    fn debug(&self, expr: &Expression<T>) -> String {
        let mut out = String::new();
        self.visit(expr, &mut out);
        out
    }

    fn pretty(&self, expr: &Expression<T>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{RenderStyle, Renderer, UnicodeRenderer};
    use b3_math::algebra::One;
    use b3_math::number::Real;

    #[test]
    fn debug_renderer_matches_display() {
        let one = Real::<i32>::one();
        let real = (
            (
                (
                    (Real::pi() + Real::e().sqrt())
                        * (Real::phi() - one.clone())
                )
                    /
                    (
                        Real::pi().sin()
                            + Real::e().ln()
                    )
            )
                .unwrap()
                +
                (
                    Real::phi()
                        .pow(one.clone() + one.clone())
                        *
                        (
                            Real::pi()
                                -
                                Real::e().exp()
                        )
                )
        )
            .sqrt()
            +
            (
                (
                    Real::pi()
                        /
                        (
                            Real::phi()
                                + Real::e()
                        )
                )
                    .unwrap()
                    .sin()
                    *
                    (
                        (
                            Real::e()
                                /
                                Real::phi()
                        )
                            .unwrap()
                            .ln()
                    )
            );

        let expr = real.as_expression();

        let renderer = UnicodeRenderer::new(RenderStyle::default());
        println!();
        println!("===============================");
        println!("{real}");
        println!("===============================");
        println!("{}", renderer.debug(expr));
        println!("===============================");
        println!("{:?}", real);
        println!("===============================");

        assert_ne!(real.to_string(),renderer.debug(expr));
    }
}
