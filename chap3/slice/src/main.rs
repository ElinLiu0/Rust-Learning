use std::io;

fn main(){
    let mut s = String::from("Hello World");
    let word = first_word(&s); // 索引5会被绑定到变量word上
    s.clear(); // 这里的clear()方法会清空当前字符串，且使用clear()方法的前提是字符串是可变的
    println!("The first word is: {}",word);
    slice_with_reference();
}

fn slice_with_reference(){
    let s = String::from("Hello World");
    let hello = &s[0..5]; // 从0开始，到5结束，不包含5
    let world = &s[6..11]; // 从6开始，到11结束，不包含11
    println!("{} {}",hello,world);
    println!("{}",&s[0..=6]); // 从0开始，到6结束，包含6
    println!("{}",&s[..=6]); // 从头开始，直到6结束，包含6
    println!("{}",&s[6..]); // 从6开始，直到结尾
    println!("{}",&s[..]); // 从头开始，直到结尾
}

fn first_word(s:&String) -> usize{
    let bytes = s.as_bytes(); // 现将字符串引用转换为字节数组
    for(i,&item) in bytes.iter().enumerate(){ // 将字节数组通过构造迭代器以及枚举化的方式，将索引和元素进行解构
        if item == b' '{ // 如果元素是空格，就返回索引
            return i;
        }     
    }
    s.len() // 如果没有空格，就返回字符串的长度
}