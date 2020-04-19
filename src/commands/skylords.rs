use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use crate::util::props::Props;
use std::error::Error;
use indexmap::IndexMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub description: String,
    pub typ: String,
    pub edition: String,
    pub rarity: String,
    pub affinity_variants: Vec<String>,
    pub orbs: Vec<String>,
    pub power_cost: Vec<u64>,
    pub weapon_type: String,
    pub charges: u64,
    pub squadsize: u64,
    pub class: String,
    pub counter: String,
    pub size: String,
    pub damage: Vec<u64>,
    pub health: Vec<u64>,
    pub abilities: Vec<Ability>,
    pub upgrades: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub typ: String,
    pub upgrade_dependency: u64,
    pub affinity_dependency: Option<String>,
    pub cost: u64,
    pub description: String,
    pub values: Option<Vec<Vec<u64>>>,
}

pub async fn carddata(props: Props) -> Result<(), Box<dyn Error>> {
    let mut t = IndexMap::new();
    t.insert("test".to_string(), String::from("123"));
    t.insert("ahri".to_string(), String::from("456"));
    t.insert("Betrix".to_string(), String::from("789"));
    t.insert("Atrox".to_string(), String::from("101"));
    t.insert("Aatrox".to_string(), String::from("111"));

    println!("{:?}", t);

    Ok(())
}