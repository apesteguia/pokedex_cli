use reqwest;
use serde::Deserialize;

pub const URL: &str = "https://pokeapi.co/api/v2/pokemon/";

impl Pokemon {
    pub async fn new(name: &str) -> Result<Pokemon, reqwest::Error> {
        let url = format!("{}{}", URL, name);

        let response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let info = response.json::<Pokemon>().await?;
            Ok(info)
        } else {
            println!("There has been an error, please check that the name or ID is correct.");
            Err(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
    pub fn show(&self) {
        println!("");
        println!("POKEMON DATA");
        println!("├─ Id: {}", self.id);
        println!("├─ Name: {}", self.name);
        println!("└─ Types:");
        for i in &self.types {
            println!("   └─ {}", i.pokemon_type.name);
        }
        println!("└─ Abilities:");
        for i in &self.abilities {
            println!("   └─ {}", i.ability.name);
        }
        println!("");
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Pokemon {
    name: String,
    id: u16,
    base_experience: i32,
    types: Vec<Types>,
    abilities: Vec<Abilities>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Types {
    //slot: u8,
    #[serde(rename = "type")]
    pokemon_type: Type,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Type {
    name: String,
    //url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Abilities {
    is_hidden: bool,
    ability: Ability, //slot: u8,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Ability {
    name: String,
    //url: String,
}
