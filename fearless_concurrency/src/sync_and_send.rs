//! Extensible Concurrency with the `Sync` and `Send` Traits
//!
//! Send: 在多线程之间转换所有权
//!
//! > The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
//!
//! 如果一个类型的所有成员都实现了 Send trait，那么该类型也实现了 Send trait。
//!
//! Sync: 在多线程之间共享可访问性
//!
//! > The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
//!
//! 如果一个类型的引用实现了 Send trait，那么该类型可以视为实现了 Sync trait；
//!
//! 与 Send 类似，如果一个类型的所有成员都实现了 Sync trait，那么该类型也实现了 Sync trait。
//!
