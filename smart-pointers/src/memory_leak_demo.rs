//! # Memory Leak Demo
//!
//! ## Reference Cycles
//!
//! 场景：有以下两个链表
//!
//! - 链表 a: 5 -> Nil
//! - 链表 b: 10 -> a
//!
//! 修改 a 的下一个节点指向 b 从而形成循环引用
//!
//! a: 5 -> 10 -> a
//! b: 10 -> 5 -> b
//!
//! ## 如何避免循环引用
//!
//! 两方面：
//!
//! 1. 通过自动化测试、code review 等方式去尽早发现代码逻辑错误导致的循环引用
//! 2. 拆分 `有所有权关系的引用` 和 `无所有权关系的引用`，并让循环引用发生在有所有权关系和无所有权关系引用之间
//!
//! 什么是“有所有权关系的引用”，什么又是“无所有权关系的引用”呢？换个说法可能会更容易理解，前者可以理解为是强引用，后者则是弱引用。
//! 弱引用的计数不会影响 Drop trait 的触发，因此任何弱引用之间形成循环后也不会导致循环引用，只要保证相关的强引用之间没有发生循环引用即可。
//!
//! - 增加 Rc 实例的强引用计数 `strong_count` 可以调用其 clone 方法
//! - 增加 Rc 实例的弱引用计数 `weak_count` 可以调用其 downgrade 方法
//!
//! ## 使用弱引用的正确姿势
//!
//! 因为弱引用指向的值并不会影响 Drop trait 的触发，这就导致在使用其引用的值的时候可能出现空指针的情况。但是不用担心，Rust 提供了一种很好
//! 的机制使其与 Option 结合，能在编译时强制你处理空指针的场景，避免对运行时造成影响。
//!
//! 调用 `Rc::downgrade` 方法增加弱引用计数，在消费弱引用的时候可以调用 upgrade 方法，该方法会返回一个 Option，引用的值存在时得到的是 `Some<T>`，否则为 `None`，
//! 而编译器会强制你处理 Option 的两种 variant，因此能够保证避免遗漏处理空指针的情况。
//!

use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
#[derive(Debug)]
enum ConsList {
    Data(i32, RefCell<Rc<ConsList>>),
    Nil,
}

#[allow(dead_code)]
impl ConsList {
    /// 获取下一个节点
    fn get_next(&self) -> Option<&RefCell<Rc<ConsList>>> {
        match self {
            ConsList::Data(_, target) => Some(target),
            ConsList::Nil => None,
        }
    }

    fn is_nil(&self) -> bool {
        match self {
            ConsList::Data(_, _) => false,
            ConsList::Nil => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_reference_cycles() {
        // 创建链表 a，并断言初始情况
        let cons_list_a = Rc::new(ConsList::Data(5, RefCell::new(Rc::new(ConsList::Nil))));

        assert_eq!(1, Rc::strong_count(&cons_list_a));

        // 断言链表 a 头结点的下一个节点为 ConsList::Nil
        if let Some(next_of_a) = cons_list_a.get_next() {
            assert!(next_of_a.borrow().is_nil());
        }

        // 创建链表 b，其头结点的下一个节点指向链表 a 的头节点
        let cons_list_b = Rc::new(ConsList::Data(10, RefCell::new(Rc::clone(&cons_list_a))));

        assert_eq!(2, Rc::strong_count(&cons_list_a));
        assert_eq!(1, Rc::strong_count(&cons_list_b));

        // 断言链表 b 头结点的下一个节点为链表 a 的头结点，只需要引用上相等即可
        if let Some(next_of_b) = cons_list_b.get_next() {
            assert!(Rc::ptr_eq(&next_of_b.borrow(), &cons_list_a));
        }

        // 修改链表 a 的下一个节点指向，从 Nil 改为链表 b 的头结点，从而形成循环引用
        if let Some(next_of_a) = cons_list_a.get_next() {
            // next_of_a.borrow_mut() 返回的是 RefMut<Rc<ConsList>> 类型，值为 Rc { value: ConsList::Nil }
            // 由于我们需要进行赋值操作，将 next_of_a 指向 cons_list_b，这一过程 Rust 会自动解引用以匹配类型
            // 首先解引用触发 Deref trait，得到的是 &Rc<ConsList>，结合 `*` 取值，最终得到 Rc<ConsList>，再将同为 Rc<ConsList> 的 cons_list_b 拷贝一份进行赋值
            *next_of_a.borrow_mut() = Rc::clone(&cons_list_b);

            assert_eq!(2, Rc::strong_count(&cons_list_a));
            assert_eq!(2, Rc::strong_count(&cons_list_b));

            // 尝试打印链表 a 时会发生 stack overflow 运行时错误，因为已经形成循环引用
            // println!("{:?}", cons_list_a);
        }
    }
}
