use std::io;
use std::cmp::Ordering;

fn if_statment(number:i32) -> bool{
    if number < 5 {
       return false
    } else if number == 5 {
        return true
    } else {
        return false
    }
}

fn match_statment(x:i32) -> bool{
    /*
        std::cmp仅接受引用类型数据！这意味着无论被引用者是变量还是字面值与否都需用
        使用&进行引用
        &操作符用于变量名前,表示创建此变量的引用
        ref用于类型名前,表示此类型的借用类型
     */
    match x.cmp(&5){
        Ordering::Greater => return false,
        Ordering::Equal => return true,
        Ordering::Less => return false
    }
}

fn main(){
    println!("{:?}",if_statment(1));
    println!("{:?}",match_statment(1));
    // 在let声明时使用if
    let condition = true;
    let number = if condition {5} else {6}; // 虽然条件表达式返回的字面值不同，但类型必须保持一致
    println!("{:?}",number);
}