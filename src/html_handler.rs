use std::io::Read;

use hyper;
use hyper::Client;
use hyper::header::Connection;

use htmlstream;
use htmlstream::HTMLTagState;


pub fn get_rss_feed(url: &str) -> String{
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

///input: String -> the given rss content could be some html tags
///this function will clear the input from unwanted tags
pub fn remove_html_tags(input: String) -> String {
    let html = input.clone();
    let mut ret = String::new();
    
    for (pos, htmltag) in htmlstream::tag_iter(&html)
             .filter(|&(ref pos, ref tag)| tag.state == HTMLTagState::Text) {
        println!("{}", &htmltag.html);
        //ret.push_str(&htmltag.html);
    }
    ret.to_string() 
}
