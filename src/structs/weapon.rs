use std::str::FromStr;
use owo_log_scrape::convert_string_to_f64;
use serde::Serialize;
use crate::structs::rank::Rank;

#[derive(Debug, Default, Serialize)]
pub struct Weapon {
    title: String,
    id: String,
    rank: Rank,
    quality: f64,
    cost: u16,

    description: String,
    passives: Vec<String>,
}
impl Weapon {
    pub fn new(title: String, texts: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut weapon = Weapon::default();
        weapon.title = title;

        for line in texts {
           if line.is_empty() { continue; }

            if let Some((key, value)) = line.split_once(":") {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "id" => weapon.id = value.to_owned(),
                    "rank" => weapon.rank = Rank::from_str(value)?,
                    "quality" => weapon.quality = convert_string_to_f64(value)?,
                    "mana cost" => weapon.cost = value.parse()?,
                    "description" => weapon.description = value.to_owned(),
                    _ => ()
                }
            }else if let Some((key, _)) = line.split_once("-") {
                let key = key.trim();
                
                weapon.passives.push(key.to_owned());
            }
        }

        Ok(weapon)
    }
}