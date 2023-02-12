use std::io;
fn main(){
    let a = 1; // 此时的a由于没有添加mut关键字，所以是不可变的
    a = 2; // 这里会报错，因为a是不可变的
}