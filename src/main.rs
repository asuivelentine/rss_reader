//! small terminal rss reader
//! written in rust

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
    
    let yml = load_yaml!("clap.yml"); //file containing valid cli arguments
    let param = App::from_yaml(yml).get_matches(); //parse the arguments

    //fetch all arguments
    let url = param.value_of("url").unwrap();
    let title_only = param.is_present("title-only");
    let num = value_t!(param.value_of("num"), usize).unwrap_or(1);
    let start = value_t!(param.value_of("start"), usize).unwrap_or(0);

    // fetch the url and get the rss string
    let content = html_handler::get_rss_feed(&url); 

    //parse that string into rss data
    let rss = Rss::from_str(&content).unwrap_or_else(|err| exit("URL's content could not be fetched") ); 
    let Rss(feed) = rss.clone(); // get the parsed content

    if title_only == true {
        decode_rss::print_titles( feed.clone() , start, num);
        return
    }

    for pos in start..start+num{
        decode_rss::print_article(feed.clone(), pos);
    }
}

fn exit(err: &str) -> ! {
    use std::process::exit;

    println!("{}", err);
    exit(1);
}
