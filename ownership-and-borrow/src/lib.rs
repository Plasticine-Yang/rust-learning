//! Ownership and Borrow
//!
//! ## 所有权转移语义 - Move 和 Copy
//!
//! - Move 语义：赋值 or 函数传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问
//! - Copy 语义：实现了 Copy trait 时，赋值 or 函数传参会使用 Copy 语义，值的所有权不会转移，而是被拷贝成新值
//! 
//! 总结：赋值 or 函数传参时，Copy 语义优先级高于 Move 语义
//!

#[cfg(test)]
mod tests {
    #[test]
    fn ownership_with_move() {
        fn calc(_: Vec<i32>) {}

        let arr = vec![1];

        // 传参在默认情况下是 Move 语义，会导致所有权转移
        calc(arr);

        // 编译报错 - 因为所有权转移后无法继续使用 arr 变量
        // println!("arr: {:?}", arr);
    }

    #[test]
    fn ownership_with_copy() {
        fn calc(_: i32) {}

        let value = 666;

        // value 是 i32 原始类型，实现了 Copy trait，此时不会使用 Move 语义，而是优先使用 Copy 语义
        // 也就是说 calc 函数得到的是 value 的一个拷贝副本，而不是 value 本身，因此所有权不会转移
        calc(value);

        // 编译通过 - 因为所有权并没有转移
        println!("value: {:?}", value);
    }
}
