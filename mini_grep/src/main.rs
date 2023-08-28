use std::{env, process};

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // eprintln! macro 会将字符串输出到 stderr 中
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 对于这种只关注 Result 的 Err 的场景，更适合用 if let 而不是 `unwrap_or_else`
    // 因为 Result 的 Ok 内的值永远是 unit type ()，对我们而言没有任何意义
    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
