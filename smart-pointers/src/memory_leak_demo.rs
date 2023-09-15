//! # Memory Leak Demo
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
    fn create_reference_cycle() {
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
