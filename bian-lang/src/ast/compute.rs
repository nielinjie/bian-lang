use super::EvalExpr;
use super::EvalExpr::*;
use super::Operator;
use super::Operator::*;

fn priority_index() -> Vec<Vec<Operator>> {
    vec![vec![Multi, Div], vec![Plus, Minus]]
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
    pub fn to_tree(&self) -> EvalExpr {
        let mut new_seq = self.clone();
        while new_seq.operators.len()> 0 {
            let operator_index = find_priority(&new_seq.operators);
            new_seq = new_seq.to_tree_step(operator_index)
        }
        new_seq.operands[0].to_owned()
    }
}
