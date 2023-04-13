use std::io;

fn main(){
    let s = String::from("Hello World"); // s进入作用域
    takes_ownership(s); // s的值移动到函数里，因为String默认是不可复制的，因此s在程序中不在有效
    let x = 5; // x进入作用域
    makes_copy(x); // 虽然x的值移动到函数里，但是i32是Copy的，所以在后面可以继续使用x
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 当some_string离开作用域时，drop函数被调用，some_string的内存被释放

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 当some_integer离开作用域时，不会有特殊操作

