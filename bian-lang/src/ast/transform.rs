use super::Expr;
use Expr::*;
use super::EvalExpr::*;
pub trait Transform{
     fn transform(exp:Expr) -> TransformResult;
}
pub type Log = String; //TODO add location/context/lineNo stuff...
pub enum TransformResult{
    NothingToDo,
    Success(Expr,Vec<Log>),
    Fail(Vec<Log>)
}
    use TransformResult::*;

struct ComputeSeqTransform;
impl Transform for ComputeSeqTransform{
     fn transform(exp:Expr) -> TransformResult{
        match exp {
            Eval(ee) =>{
                match *ee{
                    Compute(ce)=>{
                        Success(Eval(Box::new(ce.to_tree())),vec![])
                    }
                    _ => NothingToDo
                }
            },
            _ => NothingToDo,
        }
        // match exp{
        //     Eval(ref(e @ Compute(ce))) =>{
        //         Success(Eval(Box::new(ce.to_tree())),vec![])
        //     },
        //     _ => NothingToDo,
        // }
    }
}


//  trait TransformTarget{
//      fn transform(&self, transform:<dyn Transform>)->Self{
//         unimplemented!()
//     }
// }