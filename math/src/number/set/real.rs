use crate::algebra::{Add, AdditiveInverse, Div, Mul, MultiplicativeInverse, One, Sub, Zero};
use crate::number::gcd::GreatestCommonDivisor;
use crate::number::{Integer, Rational, RealError};
use b3_core::error::Result;
use b3_core::validate::Validate;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Constant<T: Integer> {
    Rational(Rational<T>),
    Pi,
    E,
    Phi,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnaryOperator {
    Neg, Sqrt, Sin, Cos, Tan, Exp, Ln,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Pow,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expression<T: Integer> {
    Constant(Constant<T>),
    Unary {
        op: UnaryOperator,
        operand: Box<Expression<T>>,
    },
    Binary {
        op: BinaryOperator,
        lhs: Box<Expression<T>>,
        rhs: Box<Expression<T>>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Real<T>
where
    T: Integer,
{
    expr: Expression<T>,
}

impl<T> Real<T>
where
    T: Integer + Clone + Div<Output = T> + GreatestCommonDivisor,
{
    pub fn pi() -> Self {
        Self { expr: Expression::Constant(Constant::Pi) }
    }
    pub fn e() -> Self {
        Self { expr: Expression::Constant(Constant::E) }
    }
    pub fn phi() -> Self {
        Self { expr: Expression::Constant(Constant::Phi) }
    }

    pub fn neg(self) -> Self { self.inverse() }
    pub fn sqrt(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Sqrt,
                operand: Box::new(self.expr),
            }
        }
    }
    pub fn sin(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Sin,
                operand: Box::new(self.expr),
            }
        }
    }
    pub fn cos(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Cos,
                operand: Box::new(self.expr),
            }
        }
    }
    pub fn tan(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Tan,
                operand: Box::new(self.expr),
            }
        }
    }
    pub fn exp(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Exp,
                operand: Box::new(self.expr),
            }
        }
    }
    pub fn ln(self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Ln,
                operand: Box::new(self.expr),
            }
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        Self {
            expr: Expression::Binary {
                op: BinaryOperator::Pow,
                lhs: Box::new(self.expr),
                rhs: Box::new(rhs.expr),
            }
        }
    }
}

impl<T> One for Real<T>
where
    T: Integer + One + Clone + Div<Output = T> + GreatestCommonDivisor,
{
    fn one() -> Self {
        Self {
            expr: Expression::Constant(
                Constant::Rational(Rational::one())
            )
        }
    }
}

impl<T> From<Rational<T>> for Real<T>
where
    T: Integer,
{
    fn from(rational: Rational<T>) -> Self {
        Self {
            expr: Expression::Constant(Constant::Rational(rational)),
        }
    }
}

impl<T> Validate for Real<T>
where
    T: Integer + Zero,
{
    type Error = RealError;

    fn validate(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<T> Add for Real<T>
where
    T: Integer + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            expr: Expression::Binary {
                op: BinaryOperator::Add,
                lhs: Box::new(self.expr),
                rhs: Box::new(rhs.expr),
            }
        }
    }
}

impl<T> Sub for Real<T>
where
    T: Integer + Add<Output = T> + Clone + Div<Output = T> + GreatestCommonDivisor,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { self + rhs.inverse() }
}

impl<T> Mul for Real<T>
where
    T: Integer + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            expr: Expression::Binary {
                op: BinaryOperator::Mul,
                lhs: Box::new(self.expr),
                rhs: Box::new(rhs.expr),
            }
        }
    }
}

impl<T> Div for Real<T>
where
    T: Integer + Mul<Output = T> + Clone + Div<Output = T> + GreatestCommonDivisor,
{
    type Output = Result<Self, RealError>;
    fn div(self, rhs: Self) -> Self::Output { Ok(self * rhs.try_inverse()?) }
}

impl<T> AdditiveInverse for Real<T>
where
    T: Integer + Clone,
{
    fn inverse(&self) -> Self {
        Self {
            expr: Expression::Unary {
                op: UnaryOperator::Neg,
                operand: Box::new(self.expr.clone()),
            },
        }
    }
    fn invert(&mut self) { *self = self.inverse() }
}

impl<T> MultiplicativeInverse for Real<T>
where
    T: Integer + One + Clone + Div<Output = T> + GreatestCommonDivisor,
{
    type Output = Self;
    type Error = RealError;

    fn try_inverse(&self) -> Result<Self::Output, Self::Error> {
        Ok(Self {
            expr: Expression::Binary {
                op: BinaryOperator::Div,
                lhs: Box::new(Expression::Constant(Constant::Rational(
                    Rational::one()
                ))),
                rhs: Box::new(self.expr.clone()),
            }
        })
    }
    fn try_invert(&mut self) -> Result<(), Self::Error> {
        *self = self.try_inverse()?;
        Ok(())
    }
}

impl<T> Display for Constant<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Rational(r) => write!(f, "{r}"),
            Constant::Pi => write!(f, "π"),
            Constant::E => write!(f, "e"),
            Constant::Phi => write!(f, "φ")
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Neg => write!(f, "-"),
            UnaryOperator::Sqrt => write!(f, "√"),
            UnaryOperator::Sin => write!(f, "sin"),
            UnaryOperator::Cos => write!(f, "cos"),
            UnaryOperator::Tan => write!(f, "tan"),
            UnaryOperator::Exp => write!(f, "exp"),
            UnaryOperator::Ln => write!(f, "ln"),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Pow => write!(f, "^"),
        }
    }
}

impl<T> Display for Expression<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(c) =>
                write!(f, "{c}"),
            Expression::Unary { op, operand } =>
                write!(f, "({op}({operand}))"),
            Expression::Binary { op, lhs, rhs } =>
                write!(f, "({lhs} {op} {rhs})")
        }
    }
}

impl<T> Display for Real<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { &_ => {
            write!(f, "{}", self.expr)
        } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_from_rational() {
        let rational = Rational::one();

        let real: Real<i32> = rational.into();

        assert!(real.validate().is_ok());
    }

    #[test]
    fn real_pi() {
        let pi = Real::<i32>::pi();

        assert_eq!(format!("{pi}"), "π");
    }

    #[test]
    fn real_e() {
        let e = Real::<i32>::e();

        assert_eq!(format!("{e}"), "e");
    }

    #[test]
    fn real_phi() {
        let phi = Real::<i32>::phi();

        assert_eq!(format!("{phi}"), "φ");
    }

    #[test]
    fn real_one() {
        let one = Real::<i32>::one();

        assert_eq!(format!("{one}"), "1/1");
    }

    #[test]
    fn neg() {
        let x = Real::<i32>::pi();

        // assert_eq!(format!("{}", -x), "-π");
    }

    #[test]
    fn sqrt() {
        let x = Real::<i32>::pi();

        assert_eq!(format!("{}", x.sqrt()), "√π");
    }

    #[test]
    fn sin() {
        let x = Real::<i32>::pi();

        assert_eq!(format!("{}", x.sin()), "sin(π)");
    }

    #[test]
    fn add() {
        let a = Real::<i32>::pi();
        let b = Real::<i32>::e();

        assert_eq!(format!("{}", a + b), "(π + e)");
    }

    #[test]
    fn sub() {
        let a = Real::<i32>::pi();
        let b = Real::<i32>::e();

        assert_eq!(format!("{}", a - b), "(π - e)");
    }

    #[test]
    fn mul() {
        let a = Real::<i32>::pi();
        let b = Real::<i32>::e();

        assert_eq!(format!("{}", a * b), "(π * e)");
    }

    #[test]
    fn div() {
        let a = Real::<i32>::pi();
        let b = Real::<i32>::e();

        assert_eq!(format!("{}", (a / b).unwrap()), "(π / e)");
    }

    #[test]
    fn pow() {
        let a = Real::<i32>::pi();
        let b = Real::<i32>::from(Rational::one());

        assert_eq!(format!("{}", a.pow(b)), "(π ^ 1/1)");
    }
}
