//! # Deref Demo
//!
//! Used for immutable dereferencing operations, like `*v`.
//!
//! ## Deref Coercion
//!
//! 当把变量作为参数传给 function 或者 method 时，Rust 会自动进行解引用转换，使其符合参数的类型，避免我们手动使用 `&` 和 `*` 去获得符合的参数类型
//!

use std::ops::Deref;

struct DerefDemo<T> {
    pub value: T,
}

#[allow(dead_code)]
impl<T> DerefDemo<T> {
    fn new(value: T) -> DerefDemo<T> {
        DerefDemo { value }
    }
}

impl<T> Deref for DerefDemo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// 解引用原始值
    #[test]
    fn deref_primitive() {
        let x = DerefDemo::new(1);

        // *x -> *(x.deref()) -> *(&i32) -> i32
        assert_eq!(1, *x);
    }

    #[test]
    fn deref_coercion() {
        fn hello(name: &str) {
            println!("{}", name);
        }

        let name = DerefDemo::new(String::from("Plasticine"));

        // &name -> &(name.deref()) -> &(&String) -> &(&str) 为了匹配 hello(name: &str) 的参数，会自动转成 &str
        hello(&name);

        // 等价的写法 - 非常麻烦，要自己通过 `&` 和 `*` 去匹配函数的参数类型
        // *name -> *(&String) -> String
        // (*name)[..] -> str
        // &(*name)[..] -> &str
        hello(&(*name)[..]);
    }
}
