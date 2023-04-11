use std::io;

fn main(){
    let x = 5;
    /*
        此时的y是一个表达式，而不是一个语句
        表达式内部通过masking的方式隐藏掉原本x=5的内存，使得代码块内部的计算结果为4
        这个过程就是表达式的计算过程，表达式返回值4并赋值给y
     */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}",y);
}