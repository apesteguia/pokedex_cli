use reqwest;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Pokemon {
    name: String,
    id: i32,
    base_experience: i32,
    types: Vec<Types>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Types {
    //slot: i32,
    #[serde(rename = "type")]
    pokemon_type: Type,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Type {
    name: String,
    //url: String,
}

pub const URL: &str = "https://pokeapi.co/api/v2/pokemon/";

impl Pokemon {
    pub async fn new(name: &str) -> Result<Pokemon, reqwest::Error> {
        let url = format!("{}{}", URL, name);

        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let info = response.json::<Pokemon>().await?;
            Ok(info)
        } else {
            println!("Ha habido un error");
            Err(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
    pub fn show(&self) {
        println!("");
        println!("POKEMON DATA");
        println!("├─ id: {}", self.id);
        println!("├─ name: {}", self.name);
        println!("└─ types:");
        for i in &self.types {
            println!("   └─ {}", i.pokemon_type.name);
        }
        println!("");
    }
}
