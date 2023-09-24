//! HTTPie implemented by Rust
//!
//! ## 简介
//!
//! [HTTPie](https://httpie.io/) 是一个类似 curl 的命令行 http client 库，由 Python 实现，该练习是使用 Rust 实现一个简易版的 HTTPie
//!
//! ## 依赖
//!  
//! 1. [clap](https://github.com/clap-rs/clap) - 命令行参数解析
//! 2. [reqwest](https://github.com/seanmonstar/reqwest) - HTTP Request
//! 3. [colored](https://github.com/colored-rs/colored) - 终端输出彩色文字
//! 4. [anyhow](https://github.com/dtolnay/anyhow) - 错误处理
//! 5. [jsonxf](https://github.com/gamache/jsonxf) - 格式化 JSON 响应体
//! 6. [mime](https://github.com/hyperium/mime) - 处理 mime 类型
//! 7. [tokio](https://github.com/tokio-rs/tokio) - 异步 io
//!

use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};
use reqwest::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(version, author = "Plasticine")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// 子命令
#[derive(Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// Get 请求子命令
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

/// Post 请求子命令
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 url
    #[clap(parse(try_from_str = parse_url))]
    url: String,

    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let args = Opts::parse();

    println!("{:?}", args);
}
