use std::io;

fn main(){
    let spaces = "   "; // spaces是一个字符串字面值
    let spaces = spaces.len(); // 此时使用let对spaces进行复写，此时spaces的类型为usize
    println!("spaces length is {}", spaces);
    let mut spaces = "   "; // spaces是一个字符串字面值
    /*
      此时对spaces进行复写，但是此时spaces的类型为usize
    */
    spaces = spaces.len();
    println!("spaces length is {}", spaces)
}