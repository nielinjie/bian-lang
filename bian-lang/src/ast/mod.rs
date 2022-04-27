#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Eval(EvalExpr),
    VarDef(String),
    Assign(String, Box<EvalExpr>),
    Return(Box<EvalExpr>),
    Composite(Vec<Expr>),
    // Block(Vec<Expr>),
}

//求值表达式可以是literal、binary、variable

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
