#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

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
    left + right
}

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
