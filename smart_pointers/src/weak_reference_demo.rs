//! # Weak Reference Demo
//!
//! ## 前言
//!
//! 我们都知道弱引用不会影响 Drop trait 的触发，但是你知道它的使用场景吗？该 Demo 会告诉你！
//!
//! 当我们设计数据结构的时候，需要保持对某个引用的所有权时，会使用强引用；反之，不需要保持所有权时会使用弱引用。什么意思呢？请看下面的例子：
//!
//! 假设现在需要实现一个树，那么最重要的就是先设计出树节点的数据结构，我们先介绍一个第一版设计，然后再在扩展它从而发现弱引用的使用场景。
//!
//! ## 第一版设计 - 支持从父节点访问所有子节点
//!
//! 每个树节点会记录自己的值 `value` 以及所有的子节点 `children`，为了简化，这里树节点的值类型就统一定为 `i32`，而所有子节点显然是有多个的，因此会设计成 Vec 类型。
//! 可是直接设计成 Vec 还不行，需要考虑以下几方面：
//!
//! 1. children 是一个 Vec，这个 Vec 应当是创建后就不可变的，但我们需要往 Vec 里添加节点，也就是说在 Vec 内部需要是可变的，也就是 interior mutability
//! 2. 为了能够做到从根节点出发去访问任意的节点，并支持用变量保存对任意节点的引用以便后续直接使用，而不用重复搜索的过程，因此需要让每个节点拥有多个所有者
//!
//! 对于第一点，我们可以通过 RefCell 去实现，对于第二点，则可以通过 Rc 去实现，那么最终类型要如何组合呢？首先给出答案 - `RefCell<Vec<Rc<Node>>>`。
//!
//! RefCell 让 children 具有内部可变性，而 `Rc<Node>` 让一个节点可以拥有多个所有者。
//!
//! 这样一来，我们就能够从一个节点中获取到它的所有子节点了，但是新的问题来了，如果给你一个子节点，你怎么获取到它的父节点呢？这时候我们的数据结构设计得修改一下了。
//!
//! ## 第二版设计 - 支持从子节点访问父节点
//!
//! 如果给 Node 结构体再加一个 parent 属性，类型为 `RefCell<Rc<Node>>`，这样会有什么问题呢？
//!
//! 父节点中可以通过 `children` 访问到子节点，子节点中也可以通过 `parent` 访问到父节点，也就是说形成了循环引用，这会导致内存泄漏！那么要如何
//! 改进呢？这就需要思考一下 `parent` 属性的类型设计是否合理了。
//!
//! 当父节点 drop 时，对应的子节点也应当 drop（但是如果有别的变量引用该节点的话仍然可以将其保留，这里只是指将其引用计数 -1），说明父节点保
//! 持对子节点的所有权，是一个强引用；而子节点 drop 时，父节点并不应该受到子节点的影响，也就是说子节点对父节点并没有持有其所有权，因此是一个
//! 弱引用。
//!
//! 因此，`parent` 的类型应当改进为 `RefCell<Weak<Node>>`
//!

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let leaf = Rc::new(Node {
            value: 1,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });

        assert_eq!(1, Rc::strong_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 2,
                children: RefCell::new(vec![Rc::clone(&leaf)]),
                parent: RefCell::new(Weak::new()),
            });

            assert_eq!(2, Rc::strong_count(&leaf));
            assert_eq!(1, Rc::strong_count(&branch));

            // 让 leaf 的 parent 指向 branch
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            assert_eq!(2, Rc::strong_count(&leaf));
            assert_eq!(1, Rc::strong_count(&branch));

            assert_eq!(0, Rc::weak_count(&leaf));
            assert_eq!(1, Rc::weak_count(&branch));
        }

        assert_eq!(1, Rc::strong_count(&leaf));
        assert_eq!(0, Rc::weak_count(&leaf));

        assert!(leaf.parent.borrow().upgrade().is_none());
    }
}
