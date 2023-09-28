//! LifeTime
//!
//! ## 值的生命周期
//!
//! ### 静态生命周期
//!
//! 如果一个值的生命周期贯穿整个进程的生命周期，那么这种生命周期就称为静态生命周期；值拥有静态生命周期时，其引用也拥有静态生命周期，用 `'static` 表示，
//! 比如 `&'static str`。以下是几种常见的具有静态生命周期的变量：
//!
//! - 全局变量
//! - 静态变量
//! - 字符串字面量
//! - 使用 `Box::leak()` 分配的指向堆内存的变量
//!
//! 相对的，如果值只在某个作用域内定义，那么其生命周期会随着作用域的结束而结束，即动态生命周期。
//!
//! ## 何时才需要为引用加上生命周期标注？
//!
//! Rust 为了减轻开发者的心智负担，会在符合一下规则的场景下自动添加生命周期标注，无需开发者手动标注：
//!
//! - 函数参数中只有一个引用类型的参数时，会自动为返回值也标注上相同的生命周期
//! - 函数参数中有多个引用类型的参数，且其中一个是 self 时，会自动为返回值标注上相同的生命周期
//!

/// 接受一个 &str 可变引用和一个分隔符，将字符串按照分隔符分割后返回第一个字符串，并将字符串引用指向后续字符串
#[allow(dead_code)]
fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[i + delimiter.len_utf8()..];

        *s = suffix;

        prefix
    } else {
        // 找不到分隔符时返回整个字符串，并将原字符串指向空串
        let prefix = *s;

        *s = "";

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifetime_with_probleam() {
        /// 函数有两个参数，如果不手动写生命周期标注的话，由于参数可能处于不同的生命周期，此时编译器无法得知返回值该以哪个生命周期为准，因此需要开发者手动标注
        fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
            if s1 > s2 {
                s1
            } else {
                s2
            }
        }

        fn get_max(s: &str) -> &str {
            max(s, "Hello")
        }

        let s1 = String::from("Hello");
        let s2 = String::from("World!");

        let result = max(&s1, &s2);

        println!("bigger one: {}", result);

        let result = get_max(&s2);

        println!("bigger one: {}", result);
    }

    #[test]
    fn use_strtok() {
        let s = "hello world".to_owned();
        let mut s_mut = s.as_str();
        let delimiter = ' ';

        let result = strtok(&mut s_mut, delimiter);

        assert_eq!("hello", result);
        assert_eq!("world", s_mut);
    }
}
