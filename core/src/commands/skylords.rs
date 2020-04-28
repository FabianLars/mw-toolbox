use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

// TODO: Convert from using Values to auto Struct conversion (=> CardSrc)
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::props::Props;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub description: String,
    pub typ: String,
    pub edition: String,
    pub rarity: String,
    pub affinity_variants: Vec<String>,
    pub orbs: Vec<String>,
    pub power_cost: Vec<i64>,
    pub weapon_type: String,
    pub charges: i64,
    pub squadsize: i64,
    pub class: String,
    pub counter: String,
    pub size: String,
    pub damage: Option<Vec<i64>>,
    pub health: Option<Vec<i64>>,
    pub abilities: Vec<Ability>,
    pub upgrades: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub typ: Option<String>,
    pub upgrade_dependency: i64,
    pub affinity_dependency: Option<String>,
    pub cost: Option<i64>,
    pub description: String,
    pub values: Option<Vec<Vec<i64>>>,
}

pub async fn carddata(props: Props) -> Result<(), Box<dyn Error>> {
    let json: Value = reqwest::get("https://cardbase.skylords.eu/Cards/GetCards").await?.json().await?;
    let json = json.get("Result").unwrap().as_array().unwrap();

    let mut result: BTreeMap<String, Card> = BTreeMap::new();

    for v in json {
        if v.get("Name").unwrap().as_str().unwrap().contains("(promo)") {
            continue;
        } else if result.contains_key(v.get("Name").unwrap().as_str().unwrap()) {
            let mut card = result.get_mut(v.get("Name").unwrap().as_str().unwrap()).unwrap();

            match v.get("Affinity").unwrap().as_i64() {
                Some(x) if x == 0 => card.affinity_variants.push("Frost".to_string()),
                Some(x) if x == 1 => card.affinity_variants.push("Fire".to_string()),
                Some(x) if x == 2 => card.affinity_variants.push("Nature".to_string()),
                Some(x) if x == 3 => card.affinity_variants.push("Shadow".to_string()),
                _ => (),
            }

            for a in v.get("Abilities").unwrap().as_array().unwrap() {
                let mut skip = false;
                for ab in &card.abilities {
                    if ab.name == a.get("Name").unwrap().as_str().unwrap().to_string() {
                        skip = true;
                    }
                }
                if skip {
                    continue;
                }
                let mut ability = Ability::default();
                ability.name = a.get("Name").unwrap().as_str().unwrap().to_string();
                if a.get("Power").unwrap().as_i64().unwrap() != 0 {
                    ability.cost = Some(a.get("Power").unwrap().as_i64().unwrap());
                }
                ability.description = a.get("Description").unwrap().as_str().unwrap().to_string();
                card.abilities.push(ability);
            }
            continue;
        }

        let mut card = Card::default();

        card.rarity = match v.get("Rarity").unwrap().as_i64() {
            Some(x) if x == 0 => String::from("Common"),
            Some(x) if x == 1 => String::from("Uncommon"),
            Some(x) if x == 2 => String::from("Rare"),
            Some(x) if x == 3 => String::from("Ultra Rare"),
            Some(x) => x.to_string(),
            None => String::new(),
        };

        card.power_cost = vec![v.get("Cost").unwrap().as_i64().unwrap(); 4];

        card.edition = match v.get("Edition").unwrap().as_i64() {
            Some(x) if x == 1 => String::from("Twilight"),
            Some(x) if x == 2 => String::from("Renegade"),
            Some(x) if x == 4 => String::from("Lost Souls"),
            Some(x) if x == 8 => String::from("Amii"),
            Some(x) => x.to_string(),
            None => String::new(),
        };

        card.typ = match v.get("Type").unwrap().as_i64() {
            Some(x) if x == 0 => String::from("Spell"),
            Some(x) if x == 2 => String::from("Unit"),
            Some(x) if x == 3 => String::from("Building"),
            Some(x) => x.to_string(),
            None => String::new(),
        };

        card.affinity_variants = match v.get("Affinity").unwrap().as_i64() {
            Some(x) if x == -1 => Vec::new(),
            Some(x) if x == 0 => vec!["Frost".to_string()],
            Some(x) if x == 1 => vec!["Fire".to_string()],
            Some(x) if x == 2 => vec!["Nature".to_string()],
            Some(x) if x == 3 => vec!["Shadow".to_string()],
            Some(x) => vec![x.to_string()],
            None => Vec::new(),
        };

        if v.get("OffenseType").unwrap().as_i64().unwrap() == 4 {
            card.weapon_type = String::from("Special");
        } else if v.get("IsRanged").unwrap().as_bool().unwrap() {
            card.weapon_type = String::from("Ranged");
        } else {
            card.weapon_type = String::from("Melee");
        }

        card.health = match v.get("Defense").unwrap().as_i64() {
            Some(x) if x == 0 => None,
            Some(x) => Some(vec![x; 4]),
            None => None,
        };

        card.damage = match v.get("Offense").unwrap().as_i64() {
            Some(x) if x == 0 => None,
            Some(x) => Some(vec![x; 4]),
            None => None,
        };

        card.size = match v.get("DefenseType").unwrap().as_i64() {
            Some(x) if x == 0 => String::from("S"),
            Some(x) if x == 1 => String::from("M"),
            Some(x) if x == 2 => String::from("L"),
            Some(x) if x == 3 => String::from("XL"),
            Some(x) if x == 4 => String::from("Building"),
            _ => String::new(),
        };

        card.counter = match v.get("OffenseType").unwrap().as_i64() {
            Some(x) if x == 0 => String::from("S"),
            Some(x) if x == 1 => String::from("M"),
            Some(x) if x == 2 => String::from("L"),
            Some(x) if x == 3 => String::from("XL"),
            Some(x) if x == 4 => String::from("Special"),
            _ => String::new(),
        };

        card.squadsize = v.get("UnitCount").unwrap().as_i64().unwrap();
        card.charges = v.get("ChargeCount").unwrap().as_i64().unwrap();
        card.class = v.get("Category").unwrap().as_str().unwrap().to_string();

        let orb_src = v.get("OrbInfo").unwrap().as_object().unwrap();
        if orb_src.get("Frost").unwrap().as_i64().unwrap() != 0 {
            for i in 0..orb_src.get("Frost").unwrap().as_i64().unwrap() {
                card.orbs.push(String::from("Frost"));
            }
        }
        if orb_src.get("Nature").unwrap().as_i64().unwrap() != 0 {
            for i in 0..orb_src.get("Nature").unwrap().as_i64().unwrap() {
                card.orbs.push(String::from("Nature"));
            }
        }
        if orb_src.get("Shadow").unwrap().as_i64().unwrap() != 0 {
            for i in 0..orb_src.get("Shadow").unwrap().as_i64().unwrap() {
                card.orbs.push(String::from("Shadow"));
            }
        }
        if orb_src.get("Fire").unwrap().as_i64().unwrap() != 0 {
            for i in 0..orb_src.get("Fire").unwrap().as_i64().unwrap() {
                card.orbs.push(String::from("Fire"));
            }
        }
        if orb_src.get("Neutral").unwrap().as_i64().unwrap() != 0 {
            for i in 0..orb_src.get("Neutral").unwrap().as_i64().unwrap() {
                card.orbs.push(String::from("Neutral"));
            }
        }

        for a in v.get("Abilities").unwrap().as_array().unwrap() {
            let mut ability = Ability::default();
            ability.name = a.get("Name").unwrap().as_str().unwrap().to_string();
            if a.get("Power").unwrap().as_i64().unwrap() != 0 {
                ability.cost = Some(a.get("Power").unwrap().as_i64().unwrap());
            }
            ability.description = a.get("Description").unwrap().as_str().unwrap().to_string();
            card.abilities.push(ability);
        }

        result.insert(v.get("Name").unwrap().as_str().unwrap().to_string(), card);
    }

    ::serde_json::to_writer(&File::create("carddata.json")?, &result)?;

    Ok(())
}