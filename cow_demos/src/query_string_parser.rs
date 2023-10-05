//! Query String Parser With `Cow<T>`
//!
//! 使用 `Cow<T>` 智能指针高效实现将 url 中的 query string 解析成 kv pair
//!
//! 功能需求：
//!
//! 1. 每解析出一个 key 或者 value，就用一个 &str 指向原始 url 中的相应位置，避免分配新的堆内存存储 String，提高效率；
//!    并将解析出来的结果用 Cow 封装起来，方便对使用方提供对 kv pair 的所有权
//! 2. 对于类似 `hello%20world` 这种需要 decode 处理的场景，就需要在堆内存上分配 String 去存储，同样将结果用 Cow 封装起来
//!

use std::borrow::Cow;

#[allow(dead_code)]
fn format_kv_pair(kv_pair: (Cow<str>, Cow<str>)) -> String {
  format!("key: {}, value: {}", format_cow(kv_pair.0), format_cow(kv_pair.1))
}

#[allow(dead_code)]
fn format_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed -- {}", v),
        Cow::Owned(v) => format!("Owned -- {}", v),
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn parse_query_string() {
        let url =
            Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
        let mut kv_pairs = url.query_pairs();

        assert_eq!(3, kv_pairs.count());

        let (mut k, v) = kv_pairs.next().unwrap();

        // 得益于 Rust 会在编译时智能匹配类型，因此虽然 k, v 的类型是 `Cow<'_, str>`，但也可以将它们视为 str 直接使用，无需手动解引用
        println!("k: {}, v: {}", k, v);

        // 修改时，k 会变为 Owned
        k.to_mut().push_str("_lala");

        println!("{}", format_kv_pair((k, v)));
        println!("{}", format_kv_pair(kv_pairs.next().unwrap()));
        println!("{}", format_kv_pair(kv_pairs.next().unwrap()));
    }
}
