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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("no url als parameter found");
    }
    let url = args[1].clone();

    let content = get_content(&url); // fetch the url and get the rss string

    let rss = Rss::from_str(&content).unwrap(); //parse that string into rss data
    let Rss(feed) = rss; // get the parsed content
    let items = feed.items; // the Title is the only thing I care about

    for item in 0..5{
        match items[item].title.clone(){
            Some(n) => println!("{}", n),
            None => panic!("cant find the data"),
        }
    }

}
