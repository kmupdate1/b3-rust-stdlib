#[derive(Clone,Debug, Copy, PartialEq, Eq)]
pub struct RenderStyle {
    /// Remove unnecessary parentheses whenever possible.
    pub minimize_parentheses: bool,
}

impl Default for RenderStyle {
    fn default() -> Self {
        Self { minimize_parentheses: true }
    }
}
