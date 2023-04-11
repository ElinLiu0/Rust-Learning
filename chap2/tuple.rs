use std::io;

fn main(){
    let tup:(i32,f64,u8) = (500,6.4,1); // 初始化一个多类型元组，此时属于构造过程
    let (x,y,z) = tup; // 解构元组，此时属于解构过程
    println!("The value of y is: {}",y);
    /*
        通过元素在元组中的顺序位置进行取值
        值得一提的是：如果在元组中使用元素下标进行索引时，不可以使用[index]，而是tup.index
    */
    println!("The value of y is: {}",tup.1)
}