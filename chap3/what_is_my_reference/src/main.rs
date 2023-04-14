use std::io;

fn main(){
    // let reference_to_nothing = dangle(); 
    /*
        从dangle()函数尝试取回一个引用,
        但是实际上dangle()函数并没有返回一个有效的引用
        因此实际运行改代码时会抛出异常
    */
    let result = no_dangle();
    /*
        当执行no_dangle()函数时，由于函数指明了返回值类型为String
        且函数最后一行的表达式return的确反悔了一个字符串类型的值，因此函数有效
        可以被正常运行
     */
    println!("The result is {}", result);
}

// fn dangle() -> &String { // dangle()函数返回一个String的引用
//     let s = String::from("hello"); // s是一个新的String

//     &s // 返回s的引用
// }

fn no_dangle() -> String{
    let s = String::from("Hello");
    return s
}