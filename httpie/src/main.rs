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

use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

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

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;

    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;

    Ok(print_resp(resp).await?)
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status());
    println!("{}\n", status);
}

/// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string(), value);
    }

    println!();
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let client = Client::new();

    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}
