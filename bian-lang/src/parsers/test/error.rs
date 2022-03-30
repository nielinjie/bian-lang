use crate::parsers::program;

#[test]
fn error_block() {
    let s = "1+2
    &%";
    let re = program(s);
    assert!(re.is_err());
}
#[test]
fn error_block_2() {
    let s = "1+2
    --";
    let re = program(s);
    assert!(re.is_err());
}

