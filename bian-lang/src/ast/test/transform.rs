use crate::ast::{
    compute::ComputeSeq,
    transform::{BlockTransform, Transform, TransformResult},
    Block, EvalExpr, Expr, Operator, Statement,
};
use EvalExpr::*;
use Expr::*;
use Operator::*;
use TransformResult::*;
#[test]
fn transform_a_block() {
    // let s = "1+2+3\n";
    let block = Block(vec![Statement(Eval(Box::new(EvalExpr::Compute(
        ComputeSeq {
            operators: vec![Plus, Minus],
            operands: vec![Literal(1), Literal(2), Literal(3)],
        },
    ))))]);
    let block_expected = Block(vec![Statement(Eval(Box::new(BinaryExpr {
        left: Box::new(BinaryExpr {
            left: Box::new(Literal(1)),
            right: Box::new(Literal(2)),
            op: Plus,
        }),
        right: Box::new(Literal(3)),
        op: Minus,
    })))]);

    println!("{:#?}", block);
    let transform = BlockTransform::transform(&block);
    match transform {
        Success(r, logs) => {
            println!("{:#?}", r);
            assert_eq!(r, block_expected);
            assert_eq!(
                logs,
                vec!["transform compute_seq to binary_expr".to_string()]
            )
        }
        o => {
            println!("{:#?}", o);
            panic!()
        }
    }
}
