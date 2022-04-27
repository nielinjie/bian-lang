use super::string_to_result;

#[test]
fn var() {
    string_to_result(
        "let a
    a=2
    return a",
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
    return a",
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
    return b",
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
    return c",
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
    return c",
        1,
    )
}


#[test]
fn var_re_assign() {
    string_to_result(
        "let a
    a=2
    a = a+1
    return a",
        3,
    )
}

#[test]
fn mid_value_expr() {
    //中间进行了一次求值，stack上多了一个值，最后结束的时候stack状态不对。
    //1. 不能以求值作为一个“语句”，没有语句这个单位？
    //2. 每个求值之后来个pop？， 最后一个不能pop。因为要返回。什么叫“最后"
    string_to_result(
        "let a
    a = 2
    a + 1
    let b = a + 3
    return b",
        5,
    )
}