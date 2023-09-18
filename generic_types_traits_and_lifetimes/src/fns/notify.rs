use std::fmt::Display;

use crate::Summary;

// 函数参数里使用 trait，可以限制某个参数必须具备 trait 所描述的特征
pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// `impl Summary` 是语法糖，其等价于使用泛型
// pub fn notify<T: Summary>(item: &T) {}

// 限制参数具备多个 trait 特征
pub fn notify_and_clone(item: &(impl Summary + Clone)) -> impl Summary + Clone {
    let cloned_item = item.clone();

    println!("{}", item.summarize());

    cloned_item
}

// 当有多个参数需要进行 trait 限制的时候
pub fn notify_with_multi_params(item1: &(impl Summary + Clone), item2: &(impl Summary + Display)) {
    let cloned_item = item1.clone();

    println!("item1: {}", cloned_item.summarize());
    println!("item2 summarize: {}", item2.summarize());
    println!("item2: {}", item2);
}

// 等价的泛型写法
// pub fn notify_with_multi_params<T: Summary + Clone, U: Summary + Display>(item1: &T, item2: &U) {}

// 上面这种多个泛型都有多个 trait 限制的写法可读性不高，Rust 提供了一种 where 语句提高这种场景下的可读性
// pub fn notify_with_multi_params<T, U>(item1: &T, item2: &U)
// where
//   T: Summary + Clone,
//   U: Summary + Display
// {}

/*
 We can also conditionally implement a trait for any type that implements another trait.
 Implementations of a trait on any type that satisfies the trait bounds are called `blanket implementations` and are extensively used in the Rust standard library.
 For example, the standard library implements the ToString trait on any type that implements the Display trait.
 The impl block in the standard library looks similar to this code:
*/
// impl<T: Display> ToString for T {
//     // --snip--
// }
