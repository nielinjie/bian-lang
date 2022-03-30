#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Int(i32),
    BinaryExpr {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    VarDef(String),
    Assign(String, Box<Expr>),
    Variable(String),

    Block(Vec<Expr>),
}

pub mod compile;

#[cfg(test)]
mod test;

