//! Fearless Concurrency
//!
//! Rust 中实现并发的四种方式：
//!
//! 1. [多线程](crate::multi_threads)
//! 2. [消息传递](crate::message_passing)
//! 3. [共享状态](crate::shared_state)
//! 4. [Sync and Send](crate::sync_and_send)
//!

pub mod message_passing;
pub mod multi_threads;
pub mod shared_state;
pub mod sync_and_send;
