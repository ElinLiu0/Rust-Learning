### match表达式详解

- 一个 `match表达式`由分支构成
- 一个分支包含一个模式和表达式开头的值预分支模式相匹配是应该执行的代码。

  - 调用match表达式时必须引入 `std::cmp::Ordering`模块，使用格式为：
    ```rust
     match <a>.cmp(&<b的引用>){
     // a > b时
     Ordering::Greater => <do something>;
     // a = b时
     Ordering::Equal => <do something>;
     // a < b时
     Ordering::Less => <do something>;
    }
    ```
- 同时match表达式可以直接处理成员内部值以进行异常处理，如：

  ```rust
  let guess:u32 = match guess.trim().parse(){
   // parse()函数的成员返回值为Ok和Err，所以match匹配
   // 这两个值就好
   Ok(num) => num, // 使用num承接Ok的返回值
   Err(_) => continue // 当输入值无法进行uint32解析时
   // 则继续让用户输入直至可以被解析器解析为uint32类型数据为止
  }
  ```
