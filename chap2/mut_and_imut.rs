use std::io;

fn main(){
    let hello = String::from("Hello World");
    /**
     * 对于Rust中的字符串，常见的初始化方式由两种：
     * 1.使用String::from()方法，from()成员方法需要一个字符串字面值作为参数，然后返回一个String类型的值
     * 2.使用String::new()方法，new()成员方法不需要参数，返回一个空的String类型的值，
     * 因此我们可以通过std::io::ReadLine()方法来为其赋值，ReadLine()方法接受一个&mut String类型的参数
     * 此时ReadLine()写入的对象是hello的引用（指针），因此hello的值会发生改变
     */
    println!("without change is {}",hello);
    let mut hello = String::from("Hello World");
    hello.push_str(",I'm here!"); // 使用String类下的push_str()方法来增加字符串
    println!("with change is {}",hello);
}