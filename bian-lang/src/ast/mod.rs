#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Eval(Box<EvalExpr>),
    VarDef(String),
    Assign(String, Box<EvalExpr>),
    Return(Box<EvalExpr>),
    Seq(Vec<Expr>),
}


#[derive(PartialEq, Debug, Clone)]
pub enum EvalExpr {
    Literal(i32),
    BinaryExpr {
        op: Operator,
        left: Box<EvalExpr>,
        right: Box<EvalExpr>,
    },
    Variable(String),
}
#[derive(PartialEq, Debug, Clone)]
pub struct Statement(pub Expr);

pub mod compile;

#[cfg(test)]
mod test;
