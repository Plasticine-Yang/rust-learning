//! Ownership and Borrow
//!
//! ## 所有权转移语义 - Move 和 Copy
//!
//! - Move 语义：赋值 or 函数传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问
//! - Copy 语义：实现了 Copy trait 时，赋值 or 函数传参会使用 Copy 语义，值的所有权不会转移，而是被拷贝成新值
//!
//! 总结：赋值 or 函数传参时，Copy 语义优先级高于 Move 语义。
//!
//! ## Borrow 语义
//!
//! **允许一个值的所有权在不发生转移的情况下，被其他上下文使用**
//!
//! Rust 中的 Borrow 语义，即借用，与其他语言的 “引用” 类似。
//!
//! Rust 中的引用实现了 Copy trait，因此在函数传参时传递引用并不会导致引用的所有权转移。
//!
//! ## 生命周期(LifeTime)
//!
//! 一个值可以有多个引用，但如果值离开了作用域导致被 drop 后，继续通过这些引用试图访问数据时就会导致 `use after free` 的问题，这个时候就需要生命周期机制来保证程序的正常运行了。
//!
//! 这个问题的核心关注点在于函数调用栈的生命周期，如果从一个生命周期长的函数中试图使用一个生命周期短的函数中的变量时是无法通过编译的，因为这会导致 `use after free` 问题。
//!
//! - 对值的约束是通过所有权机制实现的
//! - 对值的引用的约束是通过生命周期机制实现的
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

    /// 演示引用的 Copy 语义
    #[test]
    fn reference() {
        fn calc(_: &i32) {}

        let value = 666;
        let ref_value = &value;

        // 引用实现了 Copy trait，因此在传递给函数后仍能继续使用
        calc(ref_value);

        println!("ref_value: {:?}", ref_value);
    }

    #[test]
    fn borrow() {
        fn sum(value: &Vec<u32>) -> u32 {
            // value 的值的会变吗？value 的地址会变吗？

            // value 的值与外面的 borrowed_value 的值是一样的，都是 1
            // value 的地址与外面的 borrowed_value 的地址不一样，因为 borrowed_value 是引用，实现了 Copy trait，在传入时会对其进行拷贝

            // 因此函数内的 value 的值与 borrowed_value 的值一样 - 因为拷贝引用会拷贝出一个相同值的引用
            // 而 value 的地址与 borrowed_value 的地址不一样 - 因为拷贝出了另一个引用，因而引用的地址不一样
            println!("[sum] addr of value: {:p}", value);
            println!("[sum] addr of ref: {:p}", &value);

            value.iter().fold(0, |acc, x| acc + x)
        }

        let value = vec![1, 2, 3, 4]; // addr: 1
        let borrowed_value = &value; // value: 1 | addr: 2

        // 值的地址 & 值的引用的地址
        println!("addr of value: {:p}({:p})", &value, borrowed_value);
        println!(
            "addr of borrowed_value: {:p}({:p})",
            &&value, &borrowed_value
        );

        println!("sum of borrowed_value: {}", sum(borrowed_value));

        // 堆上数据的地址
        println!(
            "addr of items: [{:p}, {:p}, {:p}, {:p}]",
            &value[0], &value[1], &value[2], &value[3]
        );
    }

    /// local_ref 返回了一个 value 的引用，但 local_ref 函数作用域结束后，value 就被 drop 了，此时返回的引用指向一块被释放的内存区域
    /// 这时候外部再尝试使用时就会导致 `use after free`
    #[test]
    fn life_time() {
        // fn local_ref() -> &i32 {
        //     let value = 666;
        //     &value
        // }

        // fn run() {
        //     let ref_value = local_ref();
        //     println!("value of ref_value: {:p}", ref_value);
        // }
    }

    /// 在堆内存上使用栈内存的引用
    #[test]
    fn ref_to_stack_in_heap() {
        let mut ref_value_arr: Vec<&u32> = Vec::new();

        // &value 和 ref_value_arr 在同一作用域内，因此会随着离开作用域时一起被 drop，从而能够通过编译
        let value = 666;
        ref_value_arr.push(&value);

        // &value 和 ref_value_arr 不在同一作用域内，value 先被 drop，如果允许编译的话就会出现在该作用域之后 ref_value_arr 访问 value 的情况，导致 use after free
        // {
        //     let value = 666;
        //     ref_value_arr.push(&value);
        // }

        println!("ref_value_arr: {:?}", ref_value_arr);
    }
}
