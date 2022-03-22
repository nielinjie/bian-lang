use crate::Error;

use super::run;

#[test]
fn compiling_error() {
    let re = run("a = 2");
    assert!(re.is_err());
    match re {
        Err(Error::CompileError(s)) => assert!(!s.is_empty()),
        r => panic!("{:?}", r),
    }
}
#[test]
fn parse_error() {
    let re = run("a = = 2");
    assert!(re.is_err());
    match re {
        Err(Error::ParseError(s)) => assert!(!s.is_empty()),
        r => assert!(false,"{:?}", r),
    }
}
