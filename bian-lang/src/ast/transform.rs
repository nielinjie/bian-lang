use super::{Block, EvalExpr::*, Statement};
use super::{EvalExpr, Expr};
use Expr::*;

pub fn patch<T>(v: &Vec<T>, patch: Vec<(usize, T)>) -> Vec<T>
where
    T: Clone,
{
    let mut new_v = (*v).clone();
    patch.into_iter().for_each(|(i, item)| new_v[i] = item);
    new_v
}

pub trait Transform<T> {
    fn transform(t: &T) -> TransformResult<T>;
}
pub type Log = String; //TODO add location/context/lineNo stuff...
pub enum TransformResult<T> {
    NothingToDo,
    Success(T, Vec<Log>),
    Fail(Vec<Log>),
}
impl<T> TransformResult<T> {
    fn map<U, F>(self, f: F) -> TransformResult<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            Success(r, logs) => Success(f(r), logs),
            NothingToDo => NothingToDo,
            Fail(l) => Fail(l),
        }
    }
    // fn flatMap<U, F>(self, f: F) -> TransformResult<(Option<T>, Option<U>)>
    // where
    //     F: Fn(T) -> TransformResult<U>,
    // {
    //     match self {
    //         Success(r, logs) => {
    //             let (u, log) = f(r);
    //             Success((r, u), logs.extend(log))
    //         }
    //         NothingToDo => f(()),
    //         Fail(logs) => Fail(logs),
    //     }
    // }
    fn one_by_one<U, F>(vec: Vec<TransformResult<T>>, f: F) -> TransformResult<U>
    where
        F: Fn(Vec<(usize, T)>) -> U,
    {
        let mut new_v = vec![];
        let mut messages = vec![];
        for (i, r) in vec.into_iter().enumerate() {
            match r {
                Success(r, logs) => {
                    new_v.push((i, r));
                    messages.extend(logs)
                }
                NothingToDo => (),
                Fail(logs) => return Fail(logs),
            }
        }
        if new_v.is_empty() {
            return NothingToDo;
        }
        Success(f(new_v), messages)
    }
}

use TransformResult::*;

struct EvalExprTransform;
impl Transform<EvalExpr> for EvalExprTransform {
    fn transform(exp: &EvalExpr) -> TransformResult<EvalExpr> {
        match exp {
            Compute(ce) => Success(ce.to_tree(), vec![]),
            _ => NothingToDo,
        }
    }
}

struct ExpressionTransform;
impl Transform<Expr> for ExpressionTransform {
    fn transform(exp: &Expr) -> TransformResult<Expr> {
        match exp {
            Eval(ee) => EvalExprTransform::transform(ee).map(|e| Eval(Box::new(e))),
            Assign(s, ee) => {
                EvalExprTransform::transform(ee).map(|e| Assign(s.to_owned(), Box::new(e)))
            }
            Return(ee) => EvalExprTransform::transform(ee).map(|e| Return(Box::new(e))),
            Seq(ev) => {
                let results = ev
                    .iter()
                    .map(|e| ExpressionTransform::transform(e))
                    .collect::<Vec<_>>();
                TransformResult::one_by_one(results, |v| Seq(patch(ev, v)))
            }
            IfElse(ee, if_b, else_b) => {}
            _ => NothingToDo,
        }
    }
}

struct StatementTransform;
impl Transform<Statement> for StatementTransform {
    fn transform(stmt: &Statement) -> TransformResult<Statement> {
        let Statement(e) = stmt;
        ExpressionTransform::transform(e).map(|e| Statement(e))
    }
}
struct BlockTransform;
impl Transform<Block> for BlockTransform {
    fn transform(t: &Block) -> TransformResult<Block> {
        let statements = &t.0;
        let results = statements
            .into_iter()
            .map(|s| StatementTransform::transform(s))
            .collect::<Vec<_>>();
        TransformResult::one_by_one(results, |v| Block(patch(statements, v)))
    }
}

//  trait TransformTarget{
//      fn transform(&self, transform:<dyn Transform>)->Self{
//         unimplemented!()
//     }
// }
