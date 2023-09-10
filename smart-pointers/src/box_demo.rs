//! # `Box<T>` Demo
//!
//! `Box<T>` 用于在 heap 中存储数据，Box 本身仅仅是一个存储在 stack 中的 pointer
//!
//! ## 使用场景
//!
//! - 类型其大小无法在编译时确定，只能在运行时确定 - 常见于递归类型场景
//!   > When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
//!
//! - 不希望通过复制数据的方式转移大量数据的所有权 - 转移大量数据的所有权时会在 stack 上进行拷贝，如果将数据存储在 heap 上，那么转移所有权时只需要拷贝 pointer 即可
//!   > When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
//!
//! - 希望拥有一个实现了某个 trait 的类型而不是具体类型 - 这种场景称为 `trait object`
//!   > When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
//!

#[cfg(test)]
mod tests {
    use crate::shared::list::ConsList;

    #[test]
    fn first_try_of_box() {
        let foo = Box::new(666);
        println!("foo = {}", foo);
    }

    #[test]
    fn recursive_type() {
        let foo = ConsList::Data(
            1,
            Box::new(ConsList::Data(
                2,
                Box::new(ConsList::Data(3, Box::new(ConsList::Nil))),
            )),
        );

        println!("foo = {}", foo);
    }
}
