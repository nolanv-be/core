use crate::{Expr, Spanned, utils::ParseNested};
use proc_macro2::Span;
use syn::{
    Result,
    parse::{Parse, ParseStream},
};

/// A binary operation: `a + b`, `a += b`.
#[derive(Clone, Debug)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl ParseNested for ExprBinary {
    fn parse_nested(expr: Box<Expr>, input: ParseStream<'_>) -> Result<Self> {
        Ok(Self { left: expr, op: input.parse()?, right: input.parse()? })
    }
}

derive_parse!(ExprBinary);

impl Spanned for ExprBinary {
    fn span(&self) -> Span {
        let span = self.left.span();
        span.join(self.right.span()).unwrap_or(span)
    }

    fn set_span(&mut self, span: Span) {
        self.left.set_span(span);
        self.right.set_span(span);
    }
}

op_enum! {
    /// A binary operator: `+`, `+=`, `&`.
    pub enum BinOp {
        Le(<=),
        Ge(>=),
        Eq(==),
        Neq(!=),
        Or(||),
        And(&&),

        Assign(=),
        AddAssign(+=),
        SubAssign(-=),
        MulAssign(*=),
        DivAssign(/=),
        RemAssign(%=),
        BitAndAssign(&=),
        BitOrAssign(|=),
        BitXorAssign(^=),
        SarAssign(>>>=) peek3,
        ShlAssign(<<=),
        ShrAssign(>>=),

        Sar(>>>) peek3,
        Shr(>>),
        Shl(<<),
        BitAnd(&),
        BitOr(|),
        BitXor(^),

        Lt(<),
        Gt(>),

        Add(+),
        Sub(-),
        Pow(**) peek2,
        Mul(*),
        Div(/),
        Rem(%),
    }
}
impl BinOp {
    /// Returns `(left_binding_power, right_binding_power)` for Pratt parsing.
    ///
    /// Higher binding power means tighter binding.
    /// Left-associative: `right_bp = left_bp + 1`.
    /// Right-associative: `left_bp = right_bp + 1`.
    ///
    /// <https://docs.soliditylang.org/en/latest/cheatsheet.html#order-of-precedence-of-operators>
    pub const fn binding_power(self) -> (u8, u8) {
        match self {
            // Precedence 15: Assignment (right-associative)
            Self::Assign(_)
            | Self::AddAssign(..)
            | Self::SubAssign(..)
            | Self::MulAssign(..)
            | Self::DivAssign(..)
            | Self::RemAssign(..)
            | Self::BitAndAssign(..)
            | Self::BitOrAssign(..)
            | Self::BitXorAssign(..)
            | Self::ShlAssign(..)
            | Self::ShrAssign(..)
            | Self::SarAssign(..) => (2, 1),

            // Precedence 13: Logical OR (left-associative)
            Self::Or(..) => (5, 6),
            // Precedence 12: Logical AND (left-associative)
            Self::And(..) => (7, 8),
            // Precedence 11: Equality (left-associative)
            Self::Eq(..) | Self::Neq(..) => (9, 10),
            // Precedence 10: Comparison (left-associative)
            Self::Lt(_) | Self::Gt(_) | Self::Le(..) | Self::Ge(..) => (11, 12),
            // Precedence 9: Bitwise OR (left-associative)
            Self::BitOr(_) => (13, 14),
            // Precedence 8: Bitwise XOR (left-associative)
            Self::BitXor(_) => (15, 16),
            // Precedence 7: Bitwise AND (left-associative)
            Self::BitAnd(_) => (17, 18),
            // Precedence 6: Shift (left-associative)
            Self::Shl(..) | Self::Shr(..) | Self::Sar(..) => (19, 20),
            // Precedence 5: Addition/subtraction (left-associative)
            Self::Add(_) | Self::Sub(_) => (21, 22),
            // Precedence 4: Multiplication/division/modulo (left-associative)
            Self::Mul(_) | Self::Div(_) | Self::Rem(_) => (23, 24),
            // Precedence 3: Exponentiation (right-associative)
            Self::Pow(..) => (26, 25),
        }
    }
}
