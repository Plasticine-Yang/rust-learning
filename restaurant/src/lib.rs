// 导入模块
mod front_of_house;

// 导出模块
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
