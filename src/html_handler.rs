use std::io::Read;

use hyper;
use hyper::Client;
use hyper::header::Connection;

use htmlstream;
use htmlstream::HTMLTagState;


pub fn get_content(url: &str) -> String{
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

///in the given rss content could be some html tags
///this function will clear the input from unwanted tags
pub fn remove_html_tags(input: String) -> String {
    let html = input.clone();
    let mut ret = String::new();

    for (pos, tag) in htmlstream::tag_iter(&html) {
        if tag.state == HTMLTagState::Text { //show only the text, not the html tags
            ret.push_str(&tag.html); // add the word(s) to the return value...
        }
    }
    ret.to_string() 
}
