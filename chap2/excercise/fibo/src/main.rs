use std::io;

fn fibo(n:usize) -> Vec<i32>{
    let mut arr:Vec<i32> = Vec::new();
    for i in 1..n+1{
        if i == 1 || i == 2{
            arr.push(1);
        } else {
            /*
                如果直接使用 arr[i] = arr[i-2] + arr[i-3] 会报错
                因为对Vec或数组进行切片时，需要使用usize类型的索引
                因此，需要使用  进行类型转换
             */
            arr.push(arr[i  - 2] + arr[i  - 3]);
        }
    }
    return arr
}

fn main(){
    let mut arr:Vec<i32> = fibo(5);
    println!("{:?}",arr);
}