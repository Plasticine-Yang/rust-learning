//! # Drop Demo
//!
//! Custom code within the destructor.
//!
//! 实现了 `Drop trait` 的 `drop` 方法后，Rust 编译器会在每个函数作用域的最后加上 `drop` 方法里的这些代码，从而让类型对应的内存被自动释放
//!
//! 这种自动插入 `drop` 方法的代码的编译器行为无法禁用
//!
//! ## 在离开当前作用域之前提前 drop
//!
//! 你可能会想着直接调用 `drop` 方法不就行了么？但是在离开当前作用域之前，Rust 会自动调用一次 `drop` 方法，导致出现 `double free error`
//!
//! 虽然不能禁用 Rust 编译器在作用域最后插入 `drop` 方法中的代码，但 Rust 提供了另一种方式来帮助我们完成这一目的 - `std::mem::drop`
//!

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

/// 实现了 Drop trait 的 struct 会在 destructor 行为触发时执行 drop 方法，并触发所有属性的 destructor 行为
#[allow(dead_code)]
struct HasTwoDrop {
    one: HasDrop,
    two: HasDrop,
}

impl Drop for HasTwoDrop {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrop!");
    }
}

/// 未实现 Drop trait 的 struct 虽然在 destructor 行为触发时不会执行 drop 方法，但会触发所有属性的 destructor 行为
#[allow(dead_code)]
struct HasTwoDropWithoutDropTrait {
    one: HasDrop,
    two: HasDrop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_self() {
        let _foo = HasDrop;

        println!("Running drop_self!");
    }

    #[test]
    fn drop_self_and_all_contained_fields() {
        let _foo = HasTwoDrop {
            one: HasDrop,
            two: HasDrop,
        };

        println!("Running drop_self_and_all_contained_fields!")
    }

    #[test]
    fn drop_all_contained_fields() {
        let _foo = HasTwoDropWithoutDropTrait {
            one: HasDrop,
            two: HasDrop,
        };

        println!("Running drop_all_contained_fields!")
    }

    #[test]
    fn drop_manually() {
        let foo = HasDrop;

        println!("HasDrop crated.");
        drop(foo);
        println!("HasDrop dropped before the end of the function.");
    }
}
