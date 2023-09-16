//! # Smart Pointers
//!
//! `smart pointers` 通常使用 `struct` 实现。`smart pointers` 不同于结构体的地方在于实现了 `Deref` 和 `Drop` traits。
//!
//! - `Deref`: 具有 `*x` 这样的解引用行为 - 可参考 [deref_demo](crate::deref_demo)
//! - `Drop`: 具有 `destructor` 的行为，比如一个变量离开当前作用域之前会触发 destructor 行为，将该变量对应的内存销毁 - 可参考 [drop_demo](crate::drop_demo)
//!   - 一般来说可以不用去实现该 trait，因为 Rust 会自动触发一个结构体的所有属性的 destructor 行为
//!   - 除非是类似 `file descriptor` 或者 `socket connection` 这种，在 destructor 中需要手动关闭文件、释放连接等场景，否则大部分情况下可以由 Rust 默认处理
//!
//! 常见的 `smart pointers` 包括：
//!
//! - [`Box<T>`](crate::box_demo): for allocating values on the heap
//! - [`Rc<T>`](crate::reference_count_demo): a reference counting type that enables multiple ownership
//! - [`Ref<T>` and `RefMut<T>`](crate::ref_cell_demo): accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time
//!
//! 此外，还需要注意在组合使用具有内部可变性的 smart pointers 与 Rc 时的循环引用导致的[内存泄露问题](crate::memory_leak_demo)，比如 `RefCell<Rc<T>>`
//! 
//! 最后，还会介绍一下[弱引用](crate::weak_reference_demo)的使用场景
//!

pub mod box_demo;
pub mod deref_demo;
pub mod drop_demo;
pub mod memory_leak_demo;
pub mod ref_cell_demo;
pub mod reference_count_demo;
pub mod weak_reference_demo;
