### Rust中的循环处理

在Rust中可以使用 `loop`关键字进行循环的开始，并使用 `break`关键字跳出当前循环，例如：

```rust
let mut a:u32 = 0
loop{
 match a.cmp(10){
 Ordering::Less => a += 1,
 Ordering::Greater => {
  break;
 },
 Ordering::Equal => break,
}
}
```
