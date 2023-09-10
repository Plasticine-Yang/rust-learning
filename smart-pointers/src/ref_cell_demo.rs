//! # `RefCell<T>` Demo
//!
//! ## interior mutability 内部可变性
//!
//! 概念：允许修改不可变引用指向的数据
//! > 仅当你能保证你的代码在运行时仍能遵守 borrowing rules 时才使用 `interior mutability`
//! >
//! > 这种场景下往往是你的程序逻辑是正确的，但却违反了 Rust compiler 的规则，这时候才需要考虑一下使用 interior mutability
//!
//! 如何理解“在运行时遵守 borrowing rules”？回忆一下 borrowing rules
//!   - 任何时候都只能有一个 mutable reference 和若干个 immutable references
//!   - 所有的引用都必须有效
//!
//! 问题的关键在于如何保证引用是 immutable reference，可以从 compile time 和 runtime 两个角度去分析：
//!
//! - compile time: `Box<T>` 能在 compile time 保证引用是 immutable reference
//! - runtime: `RefCell<T>` 能在 runtime 保证引用是 immutable reference
//!
//! **`RefCell<T>` 只适用于单线程场景**
//!
//! ## `RefCell<T>`
//!
//! 提供了一些运行时方法获取对 `T` 的 mutable reference 和 immutable reference
//!

pub trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(dead_code)]
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // 方法内部需要修改 `sent_messages`，因此得用 `&mut self`，但这与 trait 的定义不符，此时就可以考虑 interior mutability
            // 也就是将 `sent_messages` 用 `RefCell<T>` 包裹起来
            // 并在需要修改 `sent_messages` 的地方通过调用 `borrow_mut` 方法获取 mutable reference 进行操作
            // 在需要读取 `sent_messages` 的地方通过调用 `borrow` 方法获取 immutable reference 进行操作
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
