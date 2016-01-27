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

pub fn remove_html_tags(input: String) -> String {
    let ret = input.clone();


    for (pos, tag) in htmlstream::tag_iter(&ret) {
        if tag.state == HTMLTagState::Text {
            print!("{}", tag.html);
        }
    }
    ret.to_string() 
}
