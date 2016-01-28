//! small terminal rss reader

#![deny(missing_docs,
         unsafe_code,
         non_camel_case_types,
         non_snake_case,
         unused_import_braces)]

extern crate hyper;
extern crate rss;
extern crate htmlstream;

mod html_handler;

use std::env;
use std::str::FromStr;

use rss::Rss;


fn print_titles(feed: rss::Channel, start: usize,  n: usize) {
    for title in feed.items.into_iter().skip(start).take(n).map(|item| item.title) {
        title.map(|t| println!("{}", t))
            .or_else(|| panic!("no valid data found"));
    }
}

fn print_descriptions(feed: rss::Channel, start: usize, n: usize){
        for description in feed.items.into_iter().skip(start).take(n).map(|item| item.description){
        description.map(|t| println!("{}" ,html_handler::remove_html_tags(t))) 
            .or_else(|| panic!("no valid data found"));
    }
}

//an article = prints the headline and the description
fn print_article(feed: rss::Channel, id: usize) {
    print_titles(feed.clone(), id, 1);
    println!("===============================================================");
    print_descriptions(feed.clone(), id, 1);
}

fn main() {
    use std::process::exit;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Usage: rss_reader [URL]");
        exit(1);
    }

    let content = html_handler::get_rss_feed(&args[1]); // fetch the url and get the rss string

    let rss = Rss::from_str(&content).unwrap(); //parse that string into rss data
    let Rss(feed) = rss.clone(); // get the parsed content
    print_article(feed, 1);
}
