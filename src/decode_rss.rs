use rss;
use rss::Rss;

use html_handler;

pub fn print_titles(feed: rss::Channel, start: usize,  n: usize) {
    for (pos, title) in feed.items.into_iter().skip(start).take(n).map(|item| item.title).enumerate() {
        title.map(|t| println!("{}\t{}", pos + start,  t))
            .or_else(|| panic!("no valid data found"));
    }
}

pub fn print_descriptions(feed: rss::Channel, start: usize, n: usize){
        for description in feed.items.into_iter().skip(start).take(n).map(|item| item.description){
        description.map(|t| println!("{}" ,html_handler::remove_html_tags(t))) 
            .or_else(|| panic!("no valid data found"));
    }
}

//an article = prints the headline and the description
pub fn print_article(feed: rss::Channel, id: usize) {
    print_titles(feed.clone(), id, 1);
    println!("===============================================================");
    print_descriptions(feed.clone(), id, 1);
    println!("---------------------------------------------------------------\n")
}
