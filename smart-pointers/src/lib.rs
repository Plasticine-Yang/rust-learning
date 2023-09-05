//! # Smart Pointers
//! Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.
//! 
//! `smart pointers` 通常使用 `struct` 实现。`smart pointers` 不同于结构体的地方在于实现了 `Deref` 和 `Drop` traits。
//! - `Deref`: 能够具有 `*x` 这样的解引用行为 - 可参考 [deref_demo](crate::deref_demo)
//!

pub mod deref_demo;
