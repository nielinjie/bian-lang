

fn main() {
    let re = bian_lang::run("let a = 1
    a = a+1
    a",None);
    println!("{:?}", re);
}




