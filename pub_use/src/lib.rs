//! # Pub Use
//!
//! 在不使用 `pub use` 的情况下，外部导入 lib.rs 中的模块时只能是：
//! ```rs
//! // main.rs
//! use pub_use::kinds::PrimaryColor;
//! use pub_use::kinds::SecondaryColor;
//! use pub_use::utils::mix;
//! use pub_use::utils::mix_secondary;
//! ```
//!
//! 但其实外部消费时并不需要关心 lib.rs 中的模块的路径是怎样的，也就是说在消费方的视角来看，更希望是这样使用：
//!
//! ```rs
//! // main.rs
//! use pub_use::PrimaryColor;
//! use pub_use::SecondaryColor;
//! use pub_use::mix;
//! use pub_use::mix_secondary;
//! ```
//!
//! `pub use` 的作用就在这里，可以在 lib.rs 内部使用 `pub use` 进行 `re-export`，从而实现上面的这种导入效果

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
pub use self::utils::mix_secondary;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!("mix two primary colors: {:?} & {:?}", c1, c2);
        SecondaryColor::Orange
    }

    /// Combines two secondary colors in equal amounts to create
    /// a primary color.
    pub fn mix_secondary(c1: SecondaryColor, c2: SecondaryColor) -> PrimaryColor {
        println!("mix two secondary colors: {:?} & {:?}", c1, c2);
        PrimaryColor::Blue
    }
}
