extern crate hyper;
extern crate rss;

use std::io::Read;
use std::env;
use std::str::FromStr;

use hyper::Client;
use hyper::header::Connection;

use rss::Rss;

fn get_content(url: &str) -> String{
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn print_titles(feed: rss::Channel, start: usize,  n: usize){
    for title in feed.items.into_iter().skip(start).take(n).map(|item| item.title) {
        title.map(|t| println!("{}", t))
            .or_else(|| panic!("cant find the data"));
    }
}

fn main() {
    use std::process::exit;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Usage: rss_reader [URL]");
        exit(1);
    }

    let content = get_content(&args[1]); // fetch the url and get the rss string

    let rss = Rss::from_str(&content).unwrap(); //parse that string into rss data
    let Rss(feed) = rss.clone(); // get the parsed content
    
    print_titles(feed, 1, 5);
}
