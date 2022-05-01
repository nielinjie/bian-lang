use crate::parsers::flows::{if_else_parser, branch_parser};

#[test]
fn simple_flow(){
    let s = "if (a) {
        b
    } else {
        c
    }";
    let ast = if_else_parser(s);
    println!("{:#?}", ast);
    assert!(ast.is_ok());
}
#[test]
fn branch_simple(){
    let s = "{\na\n}";
    let ast  = branch_parser(s);
    println!("{:#?}",ast);
    assert!(ast.is_ok());
    
}
#[test]
fn branch_new_line(){
    let s = "{
        a 
    }";
    let ast  = branch_parser(s);
    println!("{:#?}",ast);
    assert!(ast.is_ok());
}