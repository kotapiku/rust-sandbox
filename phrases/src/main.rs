extern crate phrases as sayings;

use sayings::japanese::greetings as ja_greetings;
use sayings::japanese::farewells::*;
use sayings::english::{self, farewells as en_farewells, greetings as en_greetings};

fn main() {
    println!("Hello in English: {}", en_greetings::hello());
    println!("Goodbye in English: {}", en_farewells::goodbye());
    println!("Goodbye in English: {}", english::farewells::goodbye());

    println!("Hello in japanese: {}", ja_greetings::hello());
    println!("Goodbye in japanese: {}", goodbye());
}
