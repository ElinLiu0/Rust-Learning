use std::io;

fn main(){
    let s1 = gives_ownership(); // 从gives_ownership函数中获得返回值并将其赋值给s1   
    let s2 = String::from("hello"); // s2进入作用域
    let s3 = takes_and_gives_back(s2); // s2被移动到takes_and_gives_back函数中，它也将返回值移动给s3
    /*
        因此内存移动的顺序为：s2->s3->s1
        原因是：
            - s2先被takes_and_gives_back()函数移动到函数内部，函数将s2的值作为返回值返回给s3
            - s3在得到返回值后，程序结束，s3距离作用域结束只有一行，因此s3先释放
            - LIFO原则，s1最后释放
    */
}

fn takes_and_gives_back(a_string: String) -> String { // a_string进入作用域
    a_string // 返回a_string并移交给调用函数
}

fn gives_ownership() -> String { // gives_ownership将返回一个String类型的值
    let some_string = String::from("hello"); // some_string进入作用域
    some_string // 返回some_string并移交给调用函数
}