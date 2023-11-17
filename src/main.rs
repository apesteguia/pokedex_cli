pub mod pkmn;

use std::io::{self, Write};

use pkmn::*;
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pk: Result<Pokemon, Error>;
    let mut name = String::new();

    if args.len() > 1 {
        pk = Pokemon::new(&args[1]).await;
    } else {
        println!("Enter the pokemon name or id: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).unwrap();
        pk = Pokemon::new(&name).await;
    }

    match pk {
        Ok(pokemon) => {
            pokemon.show();
        }
        Err(e) => println!("Error: {}", e),
    }
}
