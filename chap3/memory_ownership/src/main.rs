use std::io;

fn main(){
    /*
        通过使用String命名域下的from函数，在已知值上创建栈内存
     */
    let s1 = String::from("hello");
    /*
        在Rust中不支持内存拷贝的操作，取而代之的是移动语义
        此时s2在内存的操作为，完整的从s1中移动所有权，其中包含了s1中的数据指针，长度和容量
        由于s1已经失去了所有权，s1已经在栈上的内存被释放，Rust视其为无效的内存
     */
    // let s2 = s1;
    /*
        如果想让s1仍然保持堆栈完整
        性，可以调用String::clone()函数将s1的内存完整克隆给s2，
        这样s1和s2可以同时调用。
     */
    let s2 = s1.clone();
    println!("Call from s1 : {}",s1);
    println!("Call from s2 : {}",s2);
}