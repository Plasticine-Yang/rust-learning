use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Rust 中用 let 声明的变量都是 immutable 的，加上 mut 关键字使变得 mutable
        let mut guess = String::new();

        io::stdin()
            // 函数中要修改引用指向的值时需要加上 mut 关键字表明该引用指向的值可被修改
            .read_line(&mut guess)
            /*
             * - read_line 会返回 Result 枚举，该枚举有 `Ok` 和 `Err` 两个 variants(成员)
             *   - `Ok` 表示操作成功，内部包含成功时产生的值
             *   - `Err` 表示操作失败，内部包含失败的前因后果
             * - Result 实例上有一个 expect 方法
             *   - 枚举值为 `Ok` 时，expect 方法会获取 `Ok` 中的值并原样返回
             *   - 枚举值为 `Err` 时，expect 方法会让程序崩溃，并显示 expect(msg) 的 msg 参数作为错误信息
             */
            .expect("Failed to read line");

        /*
         * 将输入的字符串解析为 u32 正整数以便下面与 secret_number 进行比较
         * Rust 支持声明同名变量，官方称这个特性为 Shadowing，一般用于对同一个变量进行类型转换使用
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        /*
         * ```rust
         * let x = 5;
         * let y = 10;
         *
         * println!("x = {x} and y + 2 = {}", y + 2);
         * ```
         *
         * 输出 "x = 5 and y + 2 = 12"
         */
        println!("You guessed: {guess}");

        /*
         * 类似许多语言的 switch case 语句，但 Rust 的更强大
         * - "switch" 可以传入表达式，而不仅仅是固定值
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
