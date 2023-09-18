#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/**
 * impl 关键字用于为 struct 声明相关的方法
 */
impl Rectangle {
    /** 方法式计算面积 */
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /**
     * impl 里的函数都被称为 Associated Function (关联函数)
     *
     *   - 第一个参数类型为 Self, &Self, &mut Self 的函数都被称为 method (方法)，可用 `.` 进行调用，如 rect1.calc_area()
     *   - 第一个参数不为 Self, &Self, &mut Self 的函数则起到一个为函数约束命名空间的作用，需要用 `::` 进行调用，如 Rectangle::square(2)
     */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("area is {}", calc_area(&rect1));

    // :? 一行展示
    println!("rect1 - {:?}", rect1);

    // :#? 多行展示
    println!("rect1 - {:#?}", rect1);

    // dbg! macro 会 move rect1 的所有权，因此需要改成传入引用从而变为借用，使得 main 可以保持所有权
    dbg!(&rect1);

    println!("{}", rect1.width);

    // rect1.calc_area() 与 (&rect1).calc_area() 是等价的，Rust 会自动引用和解引用
    dbg!(rect1.calc_area());

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };

    println!("rect1 can hold rect2? -- {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? -- {}", rect1.can_hold(&rect3));

    // 调用非方法的关联函数
    let square = Rectangle::square(3);
    dbg!(square);

    // 使用 `::` 还能以函数式而非方法式的方式调用函数
    dbg!(Rectangle::calc_area(&rect1));
    dbg!(Rectangle::can_hold(&rect1, &rect2));
}

/** 函数式计算面积 */
fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
