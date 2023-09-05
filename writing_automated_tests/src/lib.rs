#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        return self.width == other.width && self.height == other.height;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.width != other.width || self.height != other.height;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    print!("add function is called -- {} + {} == {}", left, right, left + right);
    left + right
}

/**
 * - `cargo test -- --show-output` 可以在控制台输出测试过程中打印的内容，默认不输出
 * - `cargo test it_works` 运行指定的测试，测试的名称会进行模糊匹配，尽可能多地匹配符合的测试
 * - 给测试加上 `#[ignore]` 可以忽略该测试，想要只运行被忽略的测试的话只需要：`cargo test -- --ignored`
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn something_wrong() {
        panic!("Make this test fail! 哈哈哈");
    }

    #[test]
    #[should_panic]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(Rectangle::can_hold(&larger, &smaller));
        assert_eq!(&larger, &smaller);
        assert_ne!(&larger, &smaller);
    }

    #[test]
    #[should_panic]
    fn custom_error_message() {
        assert!(1 + 1 == 3, "1 + 1 == 2!");
    }

    #[test]
    fn with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Result::Ok(())
        } else {
            Result::Err(String::from("damn!"))
        }
    }
}
