use std::io;
/*
    在带有返回值的Rust函数中，通过在函数头部添加->的方式以指定
    但是我们说过，Rust是基于表达式的编程语言，这意味着：Rust函数默认的返回值将来自于
    函数体内最后一个表达式的值，且同时如果我们使用分号作为结尾时
    该表示将会变成一个语句，因此不返回任何值，这时，Rust将会返回一个空元组
 */
fn add(x:i8,y:i8) -> i8 {
    return x + y
}

fn main(){
    println!("{:?}",add(1,1));
}