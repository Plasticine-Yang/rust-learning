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
//! - `borrow`: 返回一个 `Ref<T>`，是一个运行时的 immutable reference
//! - `borrow_mut`: 返回一个 `RefMut<T>`，是一个运行时的 mutable reference
//!
//! 每调用一次 `borrow`，RefCell 内部就会在运行时对该 immutable reference 的计数就会 +1，而当相应的变量离开当前 scope 的时候，相应的计数也会 -1，`borrow_mut` 也是类似的。
//! 如果在同一作用域中尝试调用两次 `borrow_mut` 去分配两个 `RefMut<T>` 就会抛出 panic - `already borrowed: BorrowMutError`
//!
//! ## 拥有多个可变数据所有者
//!
//! 在使用 `Rc<T>` 的时候，能够分配多个 immutable reference 指向一块无法在编译时确定内存大小的数据，但如果想要分配多个 mutable reference 怎么办？
//!
//! 这时候就可以结合 `RefCell<T>` 来实现了，使用 Rc 包裹 RefCell 即可，即 `Rc<RefCell<T>>`
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
    use std::{cell::RefCell, fmt::Display, rc::Rc};

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

    #[test]
    #[should_panic]
    fn it_borrow_mut_twice_should_panic() {
        let foo: RefCell<Vec<String>> = RefCell::new(vec![]);
        let mut bar = foo.borrow_mut();
        let mut baz = foo.borrow_mut();

        bar.push(String::from("Hello"));
        baz.push(String::from("World!"));
    }

    #[test]
    fn it_multiple_owner_of_mutable_data() {
        enum ConsList {
            Data(Rc<RefCell<i32>>, Rc<ConsList>),
            Nil,
        }

        impl Display for ConsList {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fn perform_unit_of_work(cons_list: &ConsList, f: &mut std::fmt::Formatter<'_>) {
                    match cons_list {
                        ConsList::Data(value, next) => {
                            f.write_fmt(format_args!("{}", *value.borrow()))
                                .expect("format data error");

                            match **next {
                                ConsList::Data(_, _) => {
                                    f.write_str(" -> ").expect("format data error");
                                    perform_unit_of_work(next, f);
                                }
                                ConsList::Nil => perform_unit_of_work(next, f),
                            }
                        }
                        ConsList::Nil => (),
                    }
                }

                f.write_str("[").expect("format start error");

                perform_unit_of_work(self, f);

                f.write_str("]").expect("format end error");

                Ok(())
            }
        }

        let data = Rc::new(RefCell::new(5));

        let list_a = Rc::new(ConsList::Data(Rc::clone(&data), Rc::new(ConsList::Nil)));
        let list_b = Rc::new(ConsList::Data(Rc::new(RefCell::new(3)), Rc::clone(&list_a)));
        let list_c = Rc::new(ConsList::Data(Rc::new(RefCell::new(4)), Rc::clone(&list_a)));

        println!("{} before {}", "=".repeat(20), "=".repeat(20));

        println!("list_a: {}", list_a);
        println!("list_b: {}", list_b);
        println!("list_c: {}", list_c);

        // data: Rc<RefCell<i32>> 可看成是 &RefCell<i32> (smart pointer)，因此可以调用 RefCell 上的方法
        // data.borrow_mut() 返回 RefMut<i32>
        // *data.borrow_mut() == *RefMut<i32> 触发 Deref trait，得到 i32
        *data.borrow_mut() += 10;

        println!("{} after {}", "=".repeat(20), "=".repeat(20));

        println!("list_a: {}", list_a);
        println!("list_b: {}", list_b);
        println!("list_c: {}", list_c);
    }
}
