use hyper;
use hyper::Client;
use hyper::header::Connection;

use std::io::Read;

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
    ret.to_string() 
}
