#[derive(PartialEq, Debug,Clone)]
pub enum Operator {
    Plus,
    Minus,
}
#[derive(PartialEq, Debug,Clone)]
pub enum Expr {
    Int(i32),
    BinaryExpr {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryExpr {
        op:Operator,
        obj: Box<Expr>,
    },
    Block(Vec<Expr>)
}

pub mod compile;

#[cfg(test)]
mod test;

