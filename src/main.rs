extern crate minreq;
extern crate soup;

use soup::prelude::*;
use soup::Soup;
use std::env::args;

fn main() {
    // Arguments parsing
    let args = args().collect::<Vec<_>>();

    let term = args.get(1).expect("First argument must be the search term").to_string();
    let from_page = args.get(2).unwrap_or(&"1".to_owned()).parse::<u32>().unwrap();
    let to_page = args.get(3).unwrap_or(&"10".to_owned()).parse::<u32>().unwrap();

    scrape(term, from_page, to_page);
}

fn scrape(term: String, from_page: u32, to_page: u32) {
    println!("{}", term);
    for i in from_page..to_page {
        let response = minreq::get(format!("https://www.pensador.com/{}/{}", term, i)).send().unwrap();
        let html_content = response.as_str().unwrap();
        let soup = Soup::new(html_content);
        
        let thought_cards = soup.class("thought-card").find_all().collect::<Vec<_>>();
        for thought_card in thought_cards.iter() {
            let thought = thought_card.class("frase").find().unwrap().text();
            let author = thought_card.class("author-name").find().unwrap().text();
            println!("\n{} ~~ {}\n", thought, author);
        }
    }
}

