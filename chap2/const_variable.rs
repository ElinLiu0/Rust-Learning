use std::io;
use std::cmp::Ordering;
fn main(){
    // 同其他编程语言一样，Rust中也有常量的概念
    // 且常量不可添加mut关键字以使其可变
    const NUM:u32 = 10;
    println!("NUM is {}",NUM);
}