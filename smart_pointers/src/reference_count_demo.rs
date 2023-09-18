//! # Reference Count Demo
//!
//! We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.
//!
//! **`Rc<T>` 只适用于单线程场景**
//!
//! ## `foo.clone()` or `Rc::clone(&foo)` ?
//!
//! 对于 `Rc` 实例而言，两者都能实现相同的效果，但后者并没有在内存中真正拷贝数据，而是单纯地将引用计数器的值 +1，因此开销会更小
//! 官方更推荐使用 `Rc::clone(&foo)` 而不是 `foo.clone()`，因为这在排查 clone 相关的性能问题时会很有帮助，可以直接忽略 `Rc::clone()` 的代码
//!
//! > We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do. The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time. By using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count. When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.
//!
//! ## 引用了 `Rc` 的变量离开作用域时，被引用的 `Rc` 计数器会自动变更
//!
//! 当需要增加 `Rc` 计数器的值时，需要显式调用 `Rc::clone()` 来实现，但减少计数器值时无需显式调用任何函数，会随着相关变量离开作用域后自动减少计数器的值
//!

#[cfg(test)]
mod tests {
    use std::{fmt::Display, rc::Rc};

    enum ConsList {
        Data(i32, Rc<ConsList>),
        Nil,
    }

    impl Display for ConsList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fn perform_unit_of_work(cons_list: &ConsList, f: &mut std::fmt::Formatter<'_>) {
                match cons_list {
                    ConsList::Data(value, next) => {
                        f.write_fmt(format_args!("{}", value))
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

    /// 链表 a: 5 -> 10 -> Nil
    /// 链表 b: 3 -> a
    /// 链表 c: 4 -> a
    /// 存在一个数据共享的场景
    #[test]
    fn share_data() {
        // 使用 `Box` 的话会导致创建 `list_b` 时，`list_a` 的所有权转移到了 `Box::new` 方法内
        // 此时再创建 `list_c` 就会导致报错，因为 `share_data` 函数作用域中 `list_a` 已经没有对应链表节点数据的所有权了
        // let list_a = Data(5, Box::new(Data(10, Box::new(Nil))));
        // let list_b = Data(3, Box::new(list_a));
        // let list_c =  Data(4, Box::new(list_a));

        // 而改成使用 `Rc` 的话，传递的是新的引用，并不会导致所有权转移
        let list_a = Rc::new(ConsList::Data(
            5,
            Rc::new(ConsList::Data(10, Rc::new(ConsList::Nil))),
        ));

        let list_b = ConsList::Data(3, list_a.clone());
        let list_c = ConsList::Data(3, Rc::clone(&list_a));

        println!("list_a: {}", list_a);
        println!("list_a: {}", list_b);
        println!("list_a: {}", list_c);
    }

    /// 引用了 `Rc` 的变量离开作用域时，被引用的 `Rc` 计数器会自动变更
    #[test]
    #[allow(unused_variables)]
    fn decrease_count_automatically() {
        let list_a = Rc::new(ConsList::Data(
            5,
            Rc::new(ConsList::Data(10, Rc::new(ConsList::Nil))),
        ));
        println!(
            "count after creating list_a = {}",
            Rc::strong_count(&list_a)
        );

        let list_b = Rc::new(ConsList::Data(3, Rc::clone(&list_a)));
        println!(
            "count after creating list_b = {}",
            Rc::strong_count(&list_a)
        );

        {
            let list_c = Rc::new(ConsList::Data(4, Rc::clone(&list_a)));
            println!(
                "count after creating list_c = {}",
                Rc::strong_count(&list_a)
            );
        }

        println!(
            "count after list_c goes out of scope = {}",
            Rc::strong_count(&list_a)
        );
    }
}
