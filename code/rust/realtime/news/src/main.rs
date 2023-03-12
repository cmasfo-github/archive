use reqwest::blocking::get;
use select::document::Document;
use select::predicate::{Class, Name};
use select::predicate::Predicate;
use rand::prelude::SliceRandom;

fn main() {
  loop {
    // Fetch the NPR homepage and extract the news headlines and links
    let response = get("https://www.npr.org/").unwrap();
    let document = Document::from_read(response).unwrap();
    let headlines: Vec<_> = document
    .find(Name("h3").and(Class("title")))
    .map(|headline| {
      let link = headline
      .parent()
      // .and_then(|parent| parent.find(Name("a")).next())
      .and_then(|link| link.attr("href"));
      (headline.text(), link.unwrap_or_default())
    })
    .collect();
    
    // Choose a random headline and print it with its link
    if let Some((headline, link)) = headlines.choose(&mut rand::thread_rng()) {
      if *link == "" { continue; };
      println!("\n{}\n > {}", headline, link);
    } else {
      println!("Error loading headline.");
    }
    
    // Wait 5 seconds before fetching another headline
    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}