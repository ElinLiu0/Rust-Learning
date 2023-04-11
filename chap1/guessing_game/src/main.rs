use std::io;// 调用io库
use rand::Rng;// 调用rand库中的Rng模块
use std::cmp::Ordering;// 调用cmp模块中的Ordering进行比较
fn main(){
    // 生成一个1-100的随机数，.gen_range()函数的生成范围是左闭右开的
    // 其中thread_rng()函数是Rng模块中的一个函数，用于从系统中获取随机种子
    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop{
        println!("Guess the number!");
        println!("Please input your guess.");
        let mut guess = String::new();// 创建一个可变的变量guess，类型为String
        
        io::stdin() // 调用io库中的stdin函数从键盘获取输入
            .read_line(&mut guess)// 由于read_line函数的参数是一个字符串类型的引用，所以需要在变量前加上&符号
            .expect("Failed to read line"); // 如果read_line函数出错，会打印出括号中的内容
            // 但是如果这里不加.expect()，那么编译会抛出一个警告
            let guess:u32 = guess.trim().parse().expect("Please type a number!");// 将guess转换为uint32类型
        match guess.cmp(&secret_number){ 
            /**
             * cmp函数用于比较两个值，返回一个Ordering类型的枚举值
             * 在调用cmp成员方法时，操作方式如下：
             * <被比较的值>.cmp(&<带比较的值>)
             * cmp函数内部传入的待比较值得引用
             */
            Ordering::Less => println!("Too small!,Secrect is {secret_number} but your's is {guess}."),
            Ordering::Greater => println!("Too big!,Secrect is {secret_number} but your's is {guess}."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}