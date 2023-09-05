//! # Drop Demo
//!
//! Custom code within the destructor.
//!

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}

/// 实现了 Drop trait 的 struct 会在 destructor 行为触发时执行 drop 方法，并触发所有属性的 destructor 行为
#[allow(dead_code)]
struct HasTwoDrop {
    one: HasDrop,
    two: HasDrop,
}

impl Drop for HasTwoDrop {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrop!");
    }
}

/// 未实现 Drop trait 的 struct 虽然在 destructor 行为触发时不会执行 drop 方法，但会触发所有属性的 destructor 行为
#[allow(dead_code)]
struct HasTwoDropWithoutDropTrait {
    one: HasDrop,
    two: HasDrop,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_self() {
        let _foo = HasDrop;

        println!("Running drop_self!");
    }

    #[test]
    fn drop_self_and_all_contained_fields() {
        let _foo = HasTwoDrop {
            one: HasDrop,
            two: HasDrop,
        };

        println!("Running drop_self_and_all_contained_fields!")
    }

    #[test]
    fn drop_all_contained_fields() {
        let _foo = HasTwoDropWithoutDropTrait {
            one: HasDrop,
            two: HasDrop,
        };

        println!("Running drop_all_contained_fields!")
    }
}
