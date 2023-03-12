use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Name, Predicate};
use rand::prelude::SliceRandom;

fn main() {
  loop {
    // Fetch the Reuters homepage and extract the news headlines
    let response = get("https://www.reuters.com/").unwrap();
    let document = Document::from_read(response).unwrap();
    let headlines: Vec<_> = document
    .find(Name("h3").and(Name("a")))
    .map(|headline| headline.text())
    .collect();
    
    // Choose a random headline and print it
    if let Some(headline) = headlines.choose(&mut rand::thread_rng()) {
      println!("{}", headline);
    } else {
      println!("Err loading headline.");
    }
    
    // Wait 5 seconds before fetching another headline
    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}
