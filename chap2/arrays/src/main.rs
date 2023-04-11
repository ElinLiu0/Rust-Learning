use std::io;

fn main(){
    // 未指定类型的数组初始化
    let months = ["January","Feburary","March"
    ,"Aprii","May","June","July","August"
    ,"September","October","November","December"];
    // 指定类型的数组初始化
    let a:[i32;5] = [1,2,3,4,5]; // 此时a内元素均被初始化为int32
    // 当然的，也可以实现数组内只保留一个相同的值
    let b = [3;5];
    println!("{:?}",months);
    println!("{:?}",a);
    println!("{:?}",b);
    println!("{:?}",&b[0])
}