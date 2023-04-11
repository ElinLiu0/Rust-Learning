use std::io;

fn main(){
    /*
        如果不为guess声明类型，则rust编译器无法顺利推导出此时此刻guess应该被分配为什么类型的内存
        例如本例中，可为：u8，u32，i8，i32，i64，等等...
    */
    let guess:i8 = "42".parse().expect("Not a number"); 
    println!("{}",guess)
}