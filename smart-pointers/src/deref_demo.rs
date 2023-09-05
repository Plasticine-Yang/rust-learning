//! # Deref Demo
//!
//! Used for immutable dereferencing operations, like `*v`.
//!

use std::ops::Deref;

struct DerefDemo<T> {
    pub value: T,
}

impl<T> Deref for DerefDemo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn use_deref_demo() {
        let x = DerefDemo { value: 1 };
        assert_eq!(1, *x);
    }
}
