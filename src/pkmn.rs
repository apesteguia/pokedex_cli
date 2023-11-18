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
        println!("\x1b[1;34mPOKEMON DATA\x1b[0m",);
        println!("├─ \x1b[1;34mId:\x1b[0m {}", self.id);
        println!("├─ \x1b[1;34mName:\x1b[0m {}", self.name);
        println!("├─ \x1b[1;34mTypes:\x1b[0m");
        for i in &self.types {
            println!("│   └─ {}", i.pokemon_type.name);
        }
        println!("├─ \x1b[1;34mAbilities:\x1b[0m");
        for i in &self.abilities {
            if i.is_hidden {
                println!(
                    "│   └─ \x1b[1;34m\x1b[0m{} (\x1b[33mhidden\x1b[0m)",
                    i.ability.name
                );
            } else {
                println!("│   └─ \x1b[1;34m\x1b[0m{}", i.ability.name);
            }
        }
        println!(
            "├─ \x1b[1;34mStats (total):\x1b[0m {}",
            self.stats.iter().map(|s| s.base_stat).sum::<u16>()
        );

        for i in &self.stats {
            let stat_name = &i.stat.name;
            let base_stat = i.base_stat;

            let color_code = match base_stat {
                0..=50 => "\x1b[31m",      // Rojo oscuro
                51..=75 => "\x1b[93m",     // Amarillo claro
                76..=100 => "\x1b[32m",    // Verde claro \x1b[38;5;118m
                101..=300 => "\x1b[1;92m", // Verde oscuro
                _ => "\x1b[1;92m",         // Valor fuera de rango
            };

            println!("│   └─ {}{}: {}\x1b[0m", color_code, stat_name, base_stat);
        }
        println!("├─ \x1b[1;34mHeight:\x1b[0m {} m", self.height / 10.0);
        println!("├─ \x1b[1;34mWeight:\x1b[0m {} kg", self.weight);
        println!(
            "├─ \x1b[1;34mBase experience:\x1b[0m {}",
            self.base_experience
        );

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
    stats: Vec<Stats>,
    weight: f32,
    height: f32,
    order: u16,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Stats {
    stat: Stat,
    base_stat: u16,
    effort: u16,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Stat {
    name: String,
    //url: String,
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
