use super::string_to_result;

#[test]
fn var() {
    string_to_result(
        "let a
    a=2
    a",
        2,
    )
}
#[test]
fn var_more() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    a",
        2,
    )
}
#[test]
fn var_more_2() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    b",
        5,
    )
}
#[test]
fn var_three() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    let c
    c = 1+a+ b -7
    c",
        1,
    )
}
#[test]
fn var_three_short() {
    string_to_result(
        "let a
    a=2
    let b = a+3
    let c = 1+a+ b -7
    c",
        1,
    )
}
