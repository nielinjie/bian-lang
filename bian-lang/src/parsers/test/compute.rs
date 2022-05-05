use crate::ast::EvalExpr::*;
use crate::ast::Operator::*;
use crate::parsers::compute::ComputeSeq;
use crate::parsers::compute::find_priority;

#[test]
fn step_to_tree() {
    let seq = ComputeSeq {
        operators: vec![Plus],
        operands: vec![Literal(1), Literal(2)],
    };
    let stepped = ComputeSeq {
        operators: vec![],
        operands: vec![BinaryExpr {
            left: Box::new(Literal(1)),

            right: Box::new(Literal(2)),
            op: Plus,
        }],
    };
    assert_eq!(seq.to_tree_step(0), stepped)
}

#[test]
fn step_to_tree_more() {
    let seq = ComputeSeq {
        operators: vec![Plus, Minus],
        operands: vec![Literal(1), Literal(2), Literal(3)],
    };
    let stepped = ComputeSeq {
        operators: vec![Minus],
        operands: vec![
            BinaryExpr {
                left: Box::new(Literal(1)),
                right: Box::new(Literal(2)),
                op: Plus,
            },
            Literal(3),
        ],
    };
    assert_eq!(seq.to_tree_step(0), stepped)
}

#[test]
fn step_to_tree_later_first() {
    let seq = ComputeSeq {
        operators: vec![Plus, Minus],
        operands: vec![Literal(1), Literal(2), Literal(3)],
    };
    let stepped = ComputeSeq {
        operators: vec![Plus],
        operands: vec![
            Literal(1),
            BinaryExpr {
                left: Box::new(Literal(2)),
                right: Box::new(Literal(3)),
                op: Minus,
            },
        ],
    };
    assert_eq!(seq.to_tree_step(1), stepped)
}

#[test]
fn step_to_tree_go() {
    let seq = ComputeSeq {
        operators: vec![Minus],
        operands: vec![
            BinaryExpr {
                left: Box::new(Literal(1)),
                right: Box::new(Literal(2)),
                op: Plus,
            },
            Literal(3),
        ],
    };
    let stepped = ComputeSeq {
        operators: vec![],
        operands: vec![BinaryExpr {
            left: Box::new(BinaryExpr {
                left: Box::new(Literal(1)),
                right: Box::new(Literal(2)),
                op: Plus,
            }),
            right: Box::new(Literal(3)),
            op: Minus,
        }],
    };
    assert_eq!(seq.to_tree_step(0), stepped)
}

#[test]
fn find_first() {
    let seq = vec![Plus,Minus];
    let find = find_priority(&seq);
    assert_eq!(find,0);
     let seq = vec![Minus,Plus];
    let find = find_priority(&seq);
    assert_eq!(find,0);
     let seq = vec![Minus,Div,Plus];
    let find = find_priority(&seq);
    assert_eq!(find,1)
}

#[test]
#[ignore]
fn simple_to_tree() {
    let seq = ComputeSeq {
        operators: vec![Plus],
        operands: vec![Literal(1), Literal(2)],
    };
    let tree = BinaryExpr {
        left: Box::new(Literal(1)),

        right: Box::new(Literal(2)),
        op: Plus,
    };
    assert_eq!(seq.to_tree(), tree)
}

#[test]
#[ignore]
fn transform_to_tree() {
    let seq = ComputeSeq {
        operators: vec![Plus, Plus],
        operands: vec![Literal(1), Literal(2), Literal(3)],
    };
    let tree = BinaryExpr {
        left: Box::new(BinaryExpr {
            left: Box::new(Literal(1)),
            right: Box::new(Literal(2)),
            op: Plus,
        }),
        right: Box::new(Literal(3)),
        op: Plus,
    };

    assert_eq!(seq.to_tree(), tree)
}
