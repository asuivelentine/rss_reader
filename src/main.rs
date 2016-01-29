//! small terminal rss reader

#![deny(missing_docs,
         unsafe_code,
         non_camel_case_types,
         non_snake_case,
         unused_import_braces)]

#[macro_use]
extern crate clap;
extern crate hyper;
extern crate rss;
extern crate htmlstream;

mod html_handler;
mod decode_rss;

use std::str::FromStr;

use rss::Rss;


fn main() {
    use clap::App;
    
    let yml = load_yaml!("clap.yml");
    let param = App::from_yaml(yml).get_matches();

    let url = param.value_of("url").unwrap();

    let content = html_handler::get_rss_feed(&url); // fetch the url and get the rss string

    let rss = Rss::from_str(&content).unwrap(); //parse that string into rss data
    let Rss(feed) = rss.clone(); // get the parsed content
    decode_rss::print_article(feed, 1);
}
