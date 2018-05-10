extern crate mylib;

use mylib::Tweet;


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: Stirng::from("of course, as your probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
