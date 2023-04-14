fn main(){
    let my_string = String::from("hello world");
    // first_word函数可以接受String对象的切片作为参数
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word函数可以接受字符串字面值的切片作为参数
    let word = first_word(&my_string_literal[..]);
    println!("{word}");
    // 由于字符串字面字面量就是字符串切片，所以我们可以在这里直接将它传入函数，
    // 而不需要使用额外的切片语法！
    let word = first_word(my_string_literal);
    println!("{word}");
}

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}