use super::string_to_result;

#[test]
#[ignore]
fn simple_if() {
    string_to_result(
        "let a
    a=2
    let b
    if (a) {
        b =1
    }else{
        b = 2
    }
    return b",
        1,
    )
}
