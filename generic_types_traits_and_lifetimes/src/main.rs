use generic_types_traits_and_lifetimes::{NewsArticle, Summary, Tweet};

fn main() {
    let news_article = NewsArticle {
        author: String::from("Plasticine"),
        content: String::from("Hello"),
        headline: String::from("headline"),
        location: String::from("location"),
    };

    let tweet = Tweet {
        username: String::from("Plasticine"),
        content: String::from("Hi"),
        reply: true,
        retweet: false,
    };

    println!("1 new news article {}", news_article.summarize());
    println!("1 new tweet {}", tweet.summarize());
}
