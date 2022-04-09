use super::string_to_result;

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
    b",
        1,
    )
}
