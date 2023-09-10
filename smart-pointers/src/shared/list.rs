use std::fmt::Display;

/// 对于类型中存在递归类型的场景，比如这里 `ConsList` 链表节点中有成员的值又是 `ConsList`，Rust 编译器无法在编译时知道该类型的大小
/// 这种时候就可以考虑用 `Box`
/// enum ConsList {
///     Data(i32, ConsList),
///     Nil,
/// }
#[allow(dead_code)]
pub enum ConsList {
    Data(i32, Box<ConsList>),
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
