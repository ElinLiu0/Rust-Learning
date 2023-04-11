use std::io;

fn loop_method(x:u32){
    let mut counter = 0;
    loop{
        counter += 1;
        if counter == x {
            println!("Exit loop");
            break;
        }
        println!("counter = {}",counter);
    }
}
fn for_method(x:u32){
    // 在Rust中，可以使用..运算符来创建一个范围，这个范围从左边的值开始，到右边的值结束，但不包括右边的值
    for i in 1..x+1{
        println!("i = {}",i);
    }
}

fn while_method(x:u32){
    let mut counter = 0;
    while counter < x{
        counter += 1;
        println!("counter = {}",counter);
    }
}


fn main(){
    loop_method(5);
    for_method(5);
    while_method(5);
    let array = [1,2,3,4,5];
    // 使用array.iter()方法来获取数组的迭代器
    for i in array.iter(){
        println!("i = {}",i);
    }
    // 使用rev()方法来反转迭代器，rev指代reverse
    for i in (1..5).rev(){
        println!("i = {}",i);
    }
}