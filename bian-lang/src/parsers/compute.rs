use nom::{
    branch::alt,
    character::complete::one_of,
    combinator::map,
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use crate::ast::{EvalExpr, Operator};

use super::{liberal_parse, variable_parser, ws};

use EvalExpr::*;
use Operator::*;

fn priority_index() -> Vec<Vec<Operator>>{
    vec![vec![Multi,Div],vec![Plus,Minus]]
} 
pub fn find_priority(operators: &Vec<Operator>) -> usize {
    let mut find = None;
    for op in priority_index() {
        find = operators.iter().position(|o| op.contains(o));
        if find.is_some() {
            return find.unwrap();
        }
    }
    find.unwrap()
}
#[derive(Debug, Clone, PartialEq)]
pub struct ComputeSeq {
    pub operands: Vec<EvalExpr>,
    pub operators: Vec<Operator>,
}

impl ComputeSeq {
    pub fn to_tree_step(self, operator_index: usize) -> ComputeSeq {
        //left=operands[index],right =operands[index+1],op:operator[index]
        //new operands[index]=new binaryExp. operands<<1
        //operators<<1

        let mut new_operators = self.operators.clone();
        new_operators.remove(operator_index);

        let new_operand = BinaryExpr {
            left: Box::new(self.operands[operator_index].clone()),
            right: Box::new(self.operands[operator_index + 1].clone()),
            op: self.operators[operator_index].clone(),
        };

        let mut new_operands = self.operands.clone();
        new_operands[operator_index] = new_operand;
        new_operands.remove(operator_index + 1);
        ComputeSeq {
            operators: new_operators,
            operands: new_operands,
        }
    }
    pub fn to_tree(self) -> EvalExpr {
        unimplemented!("ComputeSeq::to_tree");
    }
}

pub fn operand_parser(input: &str) -> IResult<&str, EvalExpr> {
    alt((liberal_parse, variable_parser))(input)
}
pub fn operator(i: &str) -> IResult<&str, Operator> {
    map(ws(one_of("+-")), |m| match m {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        _ => unimplemented!(),
    })(i)
}
pub fn compute_parser(i: &str) -> IResult<&str, EvalExpr> {
    map(
        tuple((operand_parser, many0(pair(ws(operator), operand_parser)))),
        |(first, v)| {
            //TODO 在这里处理优先级的问题。
            //从一个operator的序列变换成一个树，以优先级为根据。
            v.into_iter().fold(first, |a, b| EvalExpr::BinaryExpr {
                op: b.0,
                left: Box::new(a),
                right: Box::new(b.1),
            })
        },
    )(i)
}
