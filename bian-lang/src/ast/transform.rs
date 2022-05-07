

use std::ops::Deref;

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
#[derive(Debug)]
pub enum TransformResult<T> {
    NothingToDo,
    Success(T, Vec<Log>),
    Fail(Vec<Log>),
}
impl<T> TransformResult<T> {
    pub fn unwrap(self) -> Option<T>{
        match self {
            Success(r,_)=> Some(r),
            NothingToDo =>None,
            _ => panic!()
        }
    }
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
    fn reduce<U>(self, b: TransformResult<U>) -> TransformResult<(Option<T>, Option<U>)> {
        match self {
            Fail(logs) => Fail(logs),
            NothingToDo => b.map(|r| (None, Some(r))),
            Success(r, logs) => match b {
                Fail(log_b) => Fail(log_b),
                NothingToDo => Success((Some(r), None), logs),
                Success(rb, log_b) => {
                    Success((Some(r), Some(rb)), logs.into_iter().chain(log_b).collect())
                }
            },
        }
    }
    fn reduce_3<U, V>(
        self,
        b: TransformResult<U>,
        c: TransformResult<V>,
    ) -> TransformResult<(Option<T>, Option<U>, Option<V>)> {
        let re = self.reduce(b).reduce(c);
        re.map(|r| {
            let (f, s) = r;
            match f {
                Some(ff) => (ff.0, ff.1, s),
                None => (None, None, s),
            }
        })
    }
    fn for_all<U, F>(vec: Vec<TransformResult<T>>, f: F) -> TransformResult<U>
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

pub struct EvalExprTransform;
 impl Transform<EvalExpr> for EvalExprTransform {
    fn transform(exp: &EvalExpr) -> TransformResult<EvalExpr> {
        match exp {
            Compute(ce) => Success(
                ce.to_tree(),
                vec!["transform compute_seq to binary_expr".to_string()],
            ),
            _ => NothingToDo,
        }
    }
}

pub struct ExpressionTransform;
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
                TransformResult::for_all(results, |v| Seq(patch(ev, v)))
            }
            IfElse(ee, if_b, else_b) => {
                let re = TransformResult::reduce_3(
                    EvalExprTransform::transform(ee),
                    BlockTransform::transform(if_b),
                    BlockTransform::transform(else_b),
                );
                re.map(|re_3| {
                    IfElse(
                        re_3.0.map(Box::new).unwrap_or(ee.clone()),
                        re_3.1.map(Box::new).unwrap_or(if_b.clone()),
                        re_3.2.map(Box::new).unwrap_or(else_b.clone()),
                    )
                })
            }
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
pub struct BlockTransform;
impl Transform<Block> for BlockTransform {
    fn transform(t: &Block) -> TransformResult<Block> {
        let statements = &t.0;
        let results = statements
            .into_iter()
            .map(|s| StatementTransform::transform(s))
            .collect::<Vec<_>>();
        TransformResult::for_all(results, |v| Block(patch(statements, v)))
    }
}

//  trait TransformTarget{
//      fn transform(&self, transform:<dyn Transform>)->Self{
//         unimplemented!()
//     }
// }
