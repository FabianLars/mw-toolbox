#![forbid(unsafe_code)]

use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use futures::{join, prelude::*, try_join};
use regex::Regex;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};
use reqwest::Client;
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{fs::File, prelude::*};

use wtools::{PathType, WikiClient};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSrc {
    pub id: i32,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummaryEntry {
    pub id: i32,
    pub name: String,
    pub alias: String,
    pub square_portrait_path: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Champ {
    pub name: String,
    pub codename: String,
    pub alias: String,
    pub id: i32,
    pub skins: Vec<Skin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skin {
    pub id: i32,
    pub id_long: i32,
    pub name: String,
}

#[cfg(feature = "riot-api")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Rotations {
    free_champion_ids: Vec<i32>,
    free_champion_ids_for_new_players: Vec<i32>,
    max_new_player_level: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreChamp {
    pub inventory_type: String,
    pub item_id: i32,
    pub item_requirements: Option<Vec<ItemReq>>,
    pub sale: Option<Sale>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReq {
    pub inventory_type: String,
    pub item_id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sale {
    pub end_date: String,
    pub prices: Vec<Price>,
    pub start_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub cost: i32,
    pub currency: String,
    pub discount: f32,
}

#[derive(Debug, Eq, PartialEq)]
struct Angebot {
    champ: String,
    skin: Option<String>,
    discount: String,
}

pub async fn champs() -> Result<()> {
    let client = Client::new();

    let fut1 = async {
        let response: Vec<SummaryEntry> = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champion-summary.json").send().await.map_err(|e| anyhow!("Couldn't get champion-summary.json: {}", e))?.json().map_err(|e| anyhow!("Couldn't convert champion-summary.json to vec: {}", e)).await?;
        Ok::<Vec<SummaryEntry>, Error>(response)
    }.map_err(|_| anyhow!("Can't get or convert champion-summary.json"));
    let fut2 = async {
        let response: HashMap<String, ChampSrc> = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await.map_err(|e| anyhow!("Couldn't get skins.json: {}", e))?.json().map_err(|e| anyhow!("Couldn't convert skins.json to hashmap: {}", e)).await?;
        Ok::<HashMap<String, ChampSrc>, Error>(response)
    }.map_err(|_| anyhow!("Can't get or convert skins.json"));

    let (summary, skins) = try_join!(fut1, fut2)?;

    let mut champions = HashMap::new();

    for c in summary.iter() {
        if c.id == -1 {
            continue;
        };
        let temp = Champ {
            name: c.name.clone(),
            codename: c.alias.to_lowercase(),
            alias: c.alias.clone(),
            id: c.id,
            skins: Vec::new(),
        };
        champions.insert(temp.id, temp);
    }

    for (s, c) in skins.iter() {
        let skinpart: Vec<char> = s.chars().rev().take(3).collect();
        let skinid = format!("{}{}{}", skinpart[2], skinpart[1], skinpart[0]).parse::<i32>()?;
        let champpart: Vec<char> = s.chars().take(c.id.to_string().len() - 3).collect();
        let champstring: String = champpart.into_iter().collect();
        let champid: i32 = champstring.parse::<i32>()?;

        let temp = Skin {
            id: skinid,
            id_long: s.parse()?,
            name: c.name.clone(),
        };

        champions.get_mut(&champid).unwrap().skins.push(temp);
    }

    File::create("champions.json")
        .await?
        .write_all(serde_json::to_string(&champions)?.as_bytes())
        .await?;

    Ok(())
}

pub async fn discounts<C: AsRef<WikiClient>>(client: C, path: PathType) -> Result<()> {
    let client = client.as_ref();
    let lockfile = std::fs::read_to_string(path.file_path()).unwrap();
    // 0: "LeagueClient", 1: PID, 2: Port, 3: Auth, 4: Protocol
    let contents = lockfile.split(':').collect::<Vec<_>>();
    let port = contents[2];
    let auth = base64::encode(format!("riot:{}", contents[3]).as_bytes());
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Basic {}", auth).parse()?);
    let unsafe_client = Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .cookie_store(true)
        .build()?;
    let json: Vec<StoreChamp> = unsafe_client
        .get(&format!(
            "https://127.0.0.1:{}/lol-store/v1/catalog?inventoryType=[\"CHAMPION\",\"CHAMPION_SKIN\"]",
            port
        ))
        .send()
        .await?
        .json()
        .await?;

    let champions_wapi: HashMap<i32, Champ> = client
        .get_external("https://api.fabianlars.de/wiki/champion")
        .await?
        .json()
        .await
        .map_err(|e| anyhow!("Can't convert champion json to hashmap: {}", e))?;

    let mut champs: Vec<Angebot> = Vec::new();
    let mut skins: Vec<Angebot> = Vec::new();
    let mut start_date = String::new();
    let mut end_date = String::new();

    for entry in &json {
        if entry.sale.is_none() {
            continue;
        }

        match entry.inventory_type.as_str() {
            "CHAMPION" => {
                let mut discount = "discount_error".to_string();
                for p in &entry.sale.as_ref().unwrap().prices {
                    if p.currency == "RP" {
                        discount = format!("{:.2}", p.discount)
                            .split('.')
                            .last()
                            .unwrap()
                            .to_string();
                    }
                }

                start_date = entry.sale.as_ref().unwrap().start_date.clone();
                end_date = entry.sale.as_ref().unwrap().end_date.clone();

                champs.push(Angebot {
                    champ: champions_wapi[&entry.item_id].name.clone(),
                    skin: None,
                    discount,
                });
            }
            "CHAMPION_SKIN" => {
                let champ_id: i32 = entry.item_requirements.as_ref().unwrap()[0].item_id;

                let mut skin = "skin_error".to_string();
                let mut discount = "discount_error".to_string();

                for s in &champions_wapi[&champ_id].skins {
                    if s.id_long == entry.item_id {
                        skin = s.name.to_string();
                    }
                }

                for p in &entry.sale.as_ref().unwrap().prices {
                    if p.currency == "RP" {
                        discount = format!("{:.2}", p.discount)
                            .split('.')
                            .last()
                            .unwrap()
                            .to_string();
                    }
                }

                skins.push(Angebot {
                    champ: champions_wapi[&champ_id].name.clone(),
                    skin: Some(skin),
                    discount,
                })
            }
            _ => {
                continue;
            }
        }
    }

    champs.sort_by(|a, b| a.champ.cmp(&b.champ));
    skins.sort_by(|a, b| a.champ.cmp(&b.champ));

    let mut angebote: String = "".to_string();

    let start_date_vec: Vec<_> = start_date.split('T').next().unwrap().split('-').collect();
    let end_date_vec: Vec<_> = end_date.split('T').next().unwrap().split('-').collect();

    start_date = format!(
        "{}.{}.{}",
        start_date_vec[2], start_date_vec[1], start_date_vec[0]
    );
    end_date = format!(
        "{}.{}.{}",
        end_date_vec[2], end_date_vec[1], end_date_vec[0]
    );

    for c in &champs {
        angebote.push_str(&format!(
            r#"
{{{{Angebot
|champ        = {}
|skin         =
|display      =
|specialprice =
|discount     = {}
}}}}"#,
            c.champ, c.discount
        ))
    }

    for s in &skins {
        angebote.push_str(&format!(
            r#"
{{{{Angebot
|champ        = {}
|skin         = {}
|display      =
|specialprice =
|discount     = {}
}}}}"#,
            s.champ,
            s.skin.as_ref().unwrap(),
            s.discount
        ))
    }

    let full_template = format!(
        r#"<div class="center">
{{{{Angebotskasten

|startdate     = {}
|enddate       = {}
<!-- nicht löschen: -->|<!--
-->{}
<!-- nicht löschen: -->}}}}
</div><noinclude>{{{{Dokumentation}}}}</noinclude>"#,
        start_date, end_date, angebote
    );

    let json: Value = client
        .request_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
            ("titles", "Vorlage:Aktuelle_Angebote"),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    client
        .request(&[
            ("action", "edit"),
            ("reason", "Nicht ganz so automatische Aktion"),
            ("bot", "1"),
            ("title", "Vorlage:Aktuelle_Angebote"),
            ("text", &full_template),
            ("token", &edit_token),
        ])
        .await?;

    Ok(())
}

#[cfg(feature = "riot-api")]
pub async fn rotation<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let client = client.as_ref();
    let riot_api_url = format!(
        "https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key={}",
        &std::env::var("RIOT_API_KEY")?
    );
    let curr_date = rename_m(chrono::Utc::today().format("%-d. %B %Y").to_string());

    let champions: HashMap<i32, Champ> = serde_json::from_str(
        &client
            .get_external_text("https://api.fabianlars.de/wiki/champion")
            .await?,
    )?;

    let rotations: Rotations = client
        .get_external(&riot_api_url)
        .await?
        .json()
        .await
        .map_err(|e| anyhow!("Can't convert rotations json to struct: {}", e))?;

    let mut rotation: Vec<String> = rotations
        .free_champion_ids
        .iter()
        .map(|x| champions[x].name.clone())
        .collect();
    let mut new_players: Vec<String> = rotations
        .free_champion_ids_for_new_players
        .iter()
        .map(|x| champions[x].name.clone())
        .collect();
    rotation.sort();
    new_players.sort();
    let rotation: String = rotation.iter().map(|x| "|".to_owned() + x).collect();
    let new_players: String = new_players.iter().map(|x| "|".to_owned() + x).collect();

    let json = client
        .request_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
            ("titles", "Vorlage:Aktuelle_Championrotation"),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    let template = format!(
        r#"{{{{Kopfzeile|[[Kostenlose Championrotation]]}}}}
<tabber>Klassisch=
{{{{#ifeq:{{{{FULLPAGENAME}}}}|Vorlage:Aktuelle Championrotation|{{{{#ifeq:{{{{#time:N|{{{{CURRENTTIMESTAMP}}}}}}}}|2|{{{{#ifexpr:{{{{#expr:{{{{#time:U|{{{{REVISIONTIMESTAMP}}}}}}}}+100000}}}}<{{{{#time:U|{{{{CURRENTTIMESTAMP}}}}}}}}|[[Kategorie:Datumskategorie Championrotation]]}}}}}}}}}}}}{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|dateto           = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
{}}}}}
|-|ARAM=
<p style="text-align: center; margin: 0 15%;">In ''Alle Zufällig''-Spielen sind zusätzlich zur normalen Rotation folgende 65 Champions immer möglich:</p>
{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|dateto           = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|lastchecked      = {}
|Aatrox|Ahri|Akali|Amumu|Annie|Ashe|Brand|Braum|Caitlyn|Cho'Gath|Darius|Draven|Ekko|Ezreal|Fiora|Fizz|Garen|Graves|Irelia|Janna|Jarvan IV.|Jax|Jhin|Jinx|Karma|Karthus|Katarina|Kayle|Kha'Zix|LeBlanc|Lee Sin|Leona|Lucian|Lulu|Lux|Malphite|Maokai|Master Yi|Miss Fortune|Mordekaiser|Morgana|Nautilus|Nidalee|Pantheon|Pyke|Quinn|Renekton|Riven|Ryze|Sivir|Sona|Soraka|Thresh|Tristana|Tryndamere|Twisted Fate|Twitch|Varus|Vayne|Veigar|Vel'Koz|Vladimir|Wukong|Xayah|Zed}}}}
|-|Neue Accounts=
<p style="text-align: center; margin: 0 15%;">Vor [[Erfahrung (Beschwörer)|Stufe 11]] haben Spieler Zugriff auf eine andere Championrotation. Diese wird seltener aktualisiert, deshalb könnte es sein, dass die folgende Liste nicht mehr korrekt ist.</p>
{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|dateto           = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|lastchecked      = {}
{}}}}}
</tabber><noinclude>{{{{Dokumentation}}}}<noinclude>"#,
        rotation, curr_date, curr_date, new_players
    );

    client
        .request(&[
            ("action", "edit"),
            ("reason", "automated action"),
            ("bot", "1"),
            ("title", "Vorlage:Aktuelle_Championrotation"),
            ("text", &template),
            ("token", &edit_token),
        ])
        .await?;

    Ok(())
}

#[cfg(feature = "riot-api")]
fn rename_m(d: String) -> String {
    if d.contains("January") {
        d.replace("January", "Januar").to_string()
    } else if d.contains("February") {
        d.replace("February", "Februar").to_string()
    } else if d.contains("March") {
        d.replace("March", "März").to_string()
    } else if d.contains("May") {
        d.replace("May", "Mai").to_string()
    } else if d.contains("June") {
        d.replace("June", "Juni").to_string()
    } else if d.contains("July") {
        d.replace("July", "Juli").to_string()
    } else if d.contains("October") {
        d.replace("October", "Oktober").to_string()
    } else if d.contains("December") {
        d.replace("December", "Dezember").to_string()
    } else {
        d
    }
}

pub async fn set<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let mut edit_token = String::new();
    let mut skin: String = String::new();
    let mut set: String = String::new();
    let mut universe: String = String::new();
    let mut icons: String = String::new();
    let mut iconsets: String = String::new();
    let mut champion: String = String::new();
    let mut tft: String = String::new();
    let client = client.as_ref();

    let fut_token = async {
        let json = client.request_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
            ("titles", "Vorlage:Set/skins.json|Vorlage:Set/sets.json|Vorlage:Set/universes.json|Vorlage:Set/icons.json|Vorlage:Set/iconsets.json|Vorlage:Set/champion.json|Vorlage:Set/TFT.json"),
        ]).await?;

        let (_i, o) = json["query"]["pages"]
            .as_object()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        edit_token = String::from(o["edittoken"].as_str().unwrap());
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get skins.json"));
    let fut_skin = async {
        skin = client.get_external_text("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").await?.replace(" ", " ").replace("Hexerei-Miss Fortune \"", "Hexerei-Miss Fortune\"");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get skins.json"));
    let fut_set = async {
        set = client.get_external_text("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skinlines.json").await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get skinlines.json"));
    let fut_universe = async {
        universe = client.get_external_text("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/universes.json").await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_icons = async {
        icons = client.get_external_text("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icons.json").await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_iconsets = async {
        iconsets = client.get_external_text("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icon-sets.json").await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_champion = async {
        let res = client
            .get_external_json("https://ddragon.leagueoflegends.com/api/versions.json")
            .await?;
        let patch_id = res.get(0).unwrap().as_str().unwrap();
        champion = client
            .get_external_text(&format!(
                "http://ddragon.leagueoflegends.com/cdn/{}/data/de_DE/champion.json",
                patch_id
            ))
            .await?
            .replace(" ", " ");
        Ok::<(), Error>(())
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_tft = async {
        tft = client
            .get_external_text("http://raw.communitydragon.org/latest/cdragon/tft/de_de.json")
            .await
            .expect("Can't get universes.json")
            .replace(" ", " ");
        Ok::<(), Error>(())
    }
    .map_err(|_| anyhow!("Can't get universes.json"));

    try_join!(
        fut_token,
        fut_skin,
        fut_set,
        fut_universe,
        fut_icons,
        fut_iconsets,
        fut_champion,
        fut_tft
    )?;

    let fut_skin = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/skins.json"),
                ("text", &skin),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get skins.json"));
    let fut_set = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/sets.json"),
                ("text", &set),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get skinlines.json"));
    let fut_universe = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/universes.json"),
                ("text", &universe),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_icons = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/icons.json"),
                ("text", &icons),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_iconsets = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/iconsets.json"),
                ("text", &iconsets),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_champion = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/champion.json"),
                ("text", &champion),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_tft = async {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/TFT.json"),
                ("text", &tft),
                ("token", &edit_token),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't get universes.json"));

    try_join!(
        fut_skin,
        fut_set,
        fut_universe,
        fut_icons,
        fut_iconsets,
        fut_champion,
        fut_tft
    )?;

    Ok(())
}

pub async fn positions<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let client = client.as_ref();
    let opgg = "https://euw.op.gg/champion/statistics";

    let (resp, resp2) = join!(
        client.get_external_text(opgg),
        client.request_json(&[
            ("action", "parse"),
            ("page", "Module:ChampionData/data"),
            ("prop", "wikitext"),
            ("format", "json")
        ])
    );

    let document = Document::from(resp?.as_str());

    let mut content = String::new();

    let champdata = resp2?;
    let champdata: String = champdata["parse"]["wikitext"]["*"]
        .as_str()
        .unwrap()
        .to_string();
    let champdata_regex = Regex::new("(?m)\\[\"op_positions\"] += .+,$")?;
    let champdata_iter = champdata_regex.split(&champdata);
    let mut positions: Vec<String> = Vec::new();

    for node in document.find(Class("champion-index__champion-item")) {
        let mut temp_positions: Vec<String> = Vec::new();

        if node
            .attr("class")
            .unwrap()
            .contains("champion-index__champion-item--TOP")
        {
            temp_positions.push("\"Oben\"".to_string());
        }
        if node
            .attr("class")
            .unwrap()
            .contains("champion-index__champion-item--MID")
        {
            temp_positions.push("\"Mitte\"".to_string());
        }
        if node
            .attr("class")
            .unwrap()
            .contains("champion-index__champion-item--ADC")
        {
            temp_positions.push("\"Unten\"".to_string());
        }
        if node
            .attr("class")
            .unwrap()
            .contains("champion-index__champion-item--SUPPORT")
        {
            temp_positions.push("\"Unterstützer\"".to_string());
        }
        if node
            .attr("class")
            .unwrap()
            .contains("champion-index__champion-item--JUNGLE")
        {
            temp_positions.push("\"Dschungel\"".to_string());
        }
        content.push_str("\n");
        positions.push(temp_positions.join(", "));
    }

    positions.push(String::new());
    let mut new_champdata: Vec<String> = Vec::new();
    for (x, y) in champdata_iter.zip(positions.into_iter()) {
        if y.is_empty() {
            new_champdata.push(x.to_string());
        } else {
            new_champdata.push(format!("{}[\"op_positions\"] = {{{}}},", x, y));
        }
    }

    let json = client
        .request_json(&[
            ("action", "query"),
            ("format", "json"),
            ("prop", "info"),
            ("intoken", "edit"),
            ("titles", "Module:ChampionData/data"),
        ])
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    client
        .request(&[
            ("action", "edit"),
            ("reason", "automated action"),
            ("bot", "1"),
            ("title", "Module:ChampionData/data"),
            ("text", &new_champdata.concat()),
            ("token", &edit_token),
        ])
        .await?;

    Ok(())
}

// programmed on demand
pub async fn random<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let client = client.as_ref();
    let champions: HashMap<i32, Champ> = serde_json::from_str(
        &client
            .get_external_text("https://api.fabianlars.de/wiki/champion")
            .await?,
    )?;

    let json: Value = client.request_json(&[
        ("action", "query"),
        ("format", "json"),
        ("prop", "info"),
        ("intoken", "edit"),
        ("titles", "Kategorie:Ahri HD-Splasharts|Kategorie:Akali HD-Splasharts|Kategorie:Alistar HD-Splasharts|Kategorie:Amumu HD-Splasharts|Kategorie:Anivia HD-Splasharts|Kategorie:Annie HD-Splasharts|Kategorie:Aphelios HD-Splasharts|Kategorie:Ashe HD-Splasharts|Kategorie:Aurelion Sol HD-Splasharts|Kategorie:Azir HD-Splasharts|Kategorie:Bard HD-Splasharts|Kategorie:Blitzcrank HD-Splasharts|Kategorie:Brand HD-Splasharts|Kategorie:Braum HD-Splasharts|Kategorie:Caitlyn HD-Splasharts|Kategorie:Camille HD-Splasharts|Kategorie:Cassiopeia HD-Splasharts|Kategorie:Cho'Gath HD-Splasharts|Kategorie:Corki HD-Splasharts|Kategorie:Darius HD-Splasharts|Kategorie:Diana HD-Splasharts|Kategorie:Dr. Mundo HD-Splasharts|Kategorie:Draven HD-Splasharts|Kategorie:Ekko HD-Splasharts|Kategorie:Elise HD-Splasharts|Kategorie:Evelynn HD-Splasharts|Kategorie:Ezreal HD-Splasharts|Kategorie:Fiddlesticks HD-Splasharts|Kategorie:Fiora HD-Splasharts|Kategorie:Fizz HD-Splasharts|Kategorie:Galio HD-Splasharts|Kategorie:Gangplank HD-Splasharts|Kategorie:Garen HD-Splasharts|Kategorie:Gnar HD-Splasharts|Kategorie:Gragas HD-Splasharts|Kategorie:Graves HD-Splasharts|Kategorie:Hecarim HD-Splasharts|Kategorie:Heimerdinger HD-Splasharts|Kategorie:Illaoi HD-Splasharts|Kategorie:Irelia HD-Splasharts|Kategorie:Ivern HD-Splasharts|Kategorie:Janna HD-Splasharts|Kategorie:Jarvan IV. HD-Splasharts|Kategorie:Jax HD-Splasharts|Kategorie:Jayce HD-Splasharts|Kategorie:Jhin HD-Splasharts|Kategorie:Jinx HD-Splasharts|Kategorie:Kai'Sa HD-Splasharts|Kategorie:Kalista HD-Splasharts|Kategorie:Karma HD-Splasharts|Kategorie:Karthus HD-Splasharts|Kategorie:Kassadin HD-Splasharts|Kategorie:Katarina HD-Splasharts|Kategorie:Kayle HD-Splasharts|Kategorie:Kayn HD-Splasharts|Kategorie:Kennen HD-Splasharts|Kategorie:Kha'Zix HD-Splasharts|Kategorie:Kindred HD-Splasharts|Kategorie:Kled HD-Splasharts|Kategorie:Kog'Maw HD-Splasharts|Kategorie:LeBlanc HD-Splasharts|Kategorie:Lee Sin HD-Splasharts|Kategorie:Leona HD-Splasharts|Kategorie:Lissandra HD-Splasharts|Kategorie:Lucian HD-Splasharts|Kategorie:Lulu HD-Splasharts|Kategorie:Lux HD-Splasharts|Kategorie:Malphite HD-Splasharts|Kategorie:Malzahar HD-Splasharts|Kategorie:Maokai HD-Splasharts|Kategorie:Master Yi HD-Splasharts|Kategorie:Miss Fortune HD-Splasharts|Kategorie:Mordekaiser HD-Splasharts|Kategorie:Morgana HD-Splasharts|Kategorie:Nami HD-Splasharts|Kategorie:Nasus HD-Splasharts|Kategorie:Nautilus HD-Splasharts|Kategorie:Neeko HD-Splasharts|Kategorie:Nidalee HD-Splasharts|Kategorie:Nocturne HD-Splasharts|Kategorie:Nunu & Willump HD-Splasharts|Kategorie:Olaf HD-Splasharts|Kategorie:Orianna HD-Splasharts|Kategorie:Ornn HD-Splasharts|Kategorie:Pantheon HD-Splasharts|Kategorie:Poppy HD-Splasharts|Kategorie:Pyke HD-Splasharts|Kategorie:Qiyana HD-Splasharts|Kategorie:Quinn HD-Splasharts|Kategorie:Rakan HD-Splasharts|Kategorie:Rammus HD-Splasharts|Kategorie:Rek'Sai HD-Splasharts|Kategorie:Renekton HD-Splasharts|Kategorie:Rengar HD-Splasharts|Kategorie:Riven HD-Splasharts|Kategorie:Rumble HD-Splasharts|Kategorie:Ryze HD-Splasharts|Kategorie:Sejuani HD-Splasharts|Kategorie:Senna HD-Splasharts|Kategorie:Sett HD-Splasharts|Kategorie:Shaco HD-Splasharts|Kategorie:Shen HD-Splasharts|Kategorie:Shyvana HD-Splasharts|Kategorie:Singed HD-Splasharts|Kategorie:Sion HD-Splasharts|Kategorie:Sivir HD-Splasharts|Kategorie:Skarner HD-Splasharts|Kategorie:Sona HD-Splasharts|Kategorie:Soraka HD-Splasharts|Kategorie:Swain HD-Splasharts|Kategorie:Sylas HD-Splasharts|Kategorie:Syndra HD-Splasharts|Kategorie:Tahm Kench HD-Splasharts|Kategorie:Taliyah HD-Splasharts|Kategorie:Talon HD-Splasharts|Kategorie:Taric HD-Splasharts|Kategorie:Teemo HD-Splasharts|Kategorie:Thresh HD-Splasharts|Kategorie:Tristana HD-Splasharts|Kategorie:Trundle HD-Splasharts|Kategorie:Tryndamere HD-Splasharts|Kategorie:Twisted Fate HD-Splasharts|Kategorie:Twitch HD-Splasharts|Kategorie:Udyr HD-Splasharts|Kategorie:Urgot HD-Splasharts|Kategorie:Varus HD-Splasharts|Kategorie:Vayne HD-Splasharts|Kategorie:Veigar HD-Splasharts|Kategorie:Vel'Koz HD-Splasharts|Kategorie:Vi HD-Splasharts|Kategorie:Viktor HD-Splasharts|Kategorie:Vladimir HD-Splasharts|Kategorie:Volibear HD-Splasharts|Kategorie:Warwick HD-Splasharts|Kategorie:Wukong HD-Splasharts|Kategorie:Xayah HD-Splasharts|Kategorie:Xerath HD-Splasharts|Kategorie:Xin Zhao HD-Splasharts|Kategorie:Yasuo HD-Splasharts|Kategorie:Yorick HD-Splasharts|Kategorie:Yuumi HD-Splasharts|Kategorie:Zac HD-Splasharts|Kategorie:Zed HD-Splasharts|Kategorie:Ziggs HD-Splasharts|Kategorie:Zilean HD-Splasharts|Kategorie:Zoe HD-Splasharts|Kategorie:Zyra HD-Splasharts")
    ]).await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token1 = String::from(o["edittoken"].as_str().unwrap());

    let json = client.request_json(&[
        ("action", "query"),
        ("format", "json"),
        ("prop", "info"),
        ("intoken", "edit"),
        ("titles", "Kategorie:Ahri Kreisbilder|Kategorie:Akali Kreisbilder|Kategorie:Alistar Kreisbilder|Kategorie:Amumu Kreisbilder|Kategorie:Anivia Kreisbilder|Kategorie:Annie Kreisbilder|Kategorie:Aphelios Kreisbilder|Kategorie:Ashe Kreisbilder|Kategorie:Aurelion Sol Kreisbilder|Kategorie:Azir Kreisbilder|Kategorie:Bard Kreisbilder|Kategorie:Blitzcrank Kreisbilder|Kategorie:Brand Kreisbilder|Kategorie:Braum Kreisbilder|Kategorie:Caitlyn Kreisbilder|Kategorie:Camille Kreisbilder|Kategorie:Cassiopeia Kreisbilder|Kategorie:Cho'Gath Kreisbilder|Kategorie:Corki Kreisbilder|Kategorie:Darius Kreisbilder|Kategorie:Diana Kreisbilder|Kategorie:Dr. Mundo Kreisbilder|Kategorie:Draven Kreisbilder|Kategorie:Ekko Kreisbilder|Kategorie:Elise Kreisbilder|Kategorie:Evelynn Kreisbilder|Kategorie:Ezreal Kreisbilder|Kategorie:Fiddlesticks Kreisbilder|Kategorie:Fiora Kreisbilder|Kategorie:Fizz Kreisbilder|Kategorie:Galio Kreisbilder|Kategorie:Gangplank Kreisbilder|Kategorie:Garen Kreisbilder|Kategorie:Gnar Kreisbilder|Kategorie:Gragas Kreisbilder|Kategorie:Graves Kreisbilder|Kategorie:Hecarim Kreisbilder|Kategorie:Heimerdinger Kreisbilder|Kategorie:Illaoi Kreisbilder|Kategorie:Irelia Kreisbilder|Kategorie:Ivern Kreisbilder|Kategorie:Janna Kreisbilder|Kategorie:Jarvan IV. Kreisbilder|Kategorie:Jax Kreisbilder|Kategorie:Jayce Kreisbilder|Kategorie:Jhin Kreisbilder|Kategorie:Jinx Kreisbilder|Kategorie:Kai'Sa Kreisbilder|Kategorie:Kalista Kreisbilder|Kategorie:Karma Kreisbilder|Kategorie:Karthus Kreisbilder|Kategorie:Kassadin Kreisbilder|Kategorie:Katarina Kreisbilder|Kategorie:Kayle Kreisbilder|Kategorie:Kayn Kreisbilder|Kategorie:Kennen Kreisbilder|Kategorie:Kha'Zix Kreisbilder|Kategorie:Kindred Kreisbilder|Kategorie:Kled Kreisbilder|Kategorie:Kog'Maw Kreisbilder|Kategorie:LeBlanc Kreisbilder|Kategorie:Lee Sin Kreisbilder|Kategorie:Leona Kreisbilder|Kategorie:Lissandra Kreisbilder|Kategorie:Lucian Kreisbilder|Kategorie:Lulu Kreisbilder|Kategorie:Lux Kreisbilder|Kategorie:Malphite Kreisbilder|Kategorie:Malzahar Kreisbilder|Kategorie:Maokai Kreisbilder|Kategorie:Master Yi Kreisbilder|Kategorie:Miss Fortune Kreisbilder|Kategorie:Mordekaiser Kreisbilder|Kategorie:Morgana Kreisbilder|Kategorie:Nami Kreisbilder|Kategorie:Nasus Kreisbilder|Kategorie:Nautilus Kreisbilder|Kategorie:Neeko Kreisbilder|Kategorie:Nidalee Kreisbilder|Kategorie:Nocturne Kreisbilder|Kategorie:Nunu & Willump Kreisbilder|Kategorie:Olaf Kreisbilder|Kategorie:Orianna Kreisbilder|Kategorie:Ornn Kreisbilder|Kategorie:Pantheon Kreisbilder|Kategorie:Poppy Kreisbilder|Kategorie:Pyke Kreisbilder|Kategorie:Qiyana Kreisbilder|Kategorie:Quinn Kreisbilder|Kategorie:Rakan Kreisbilder|Kategorie:Rammus Kreisbilder|Kategorie:Rek'Sai Kreisbilder|Kategorie:Renekton Kreisbilder|Kategorie:Rengar Kreisbilder|Kategorie:Riven Kreisbilder|Kategorie:Rumble Kreisbilder|Kategorie:Ryze Kreisbilder|Kategorie:Sejuani Kreisbilder|Kategorie:Senna Kreisbilder|Kategorie:Sett Kreisbilder|Kategorie:Shaco Kreisbilder|Kategorie:Shen Kreisbilder|Kategorie:Shyvana Kreisbilder|Kategorie:Singed Kreisbilder|Kategorie:Sion Kreisbilder|Kategorie:Sivir Kreisbilder|Kategorie:Skarner Kreisbilder|Kategorie:Sona Kreisbilder|Kategorie:Soraka Kreisbilder|Kategorie:Swain Kreisbilder|Kategorie:Sylas Kreisbilder|Kategorie:Syndra Kreisbilder|Kategorie:Tahm Kench Kreisbilder|Kategorie:Taliyah Kreisbilder|Kategorie:Talon Kreisbilder|Kategorie:Taric Kreisbilder|Kategorie:Teemo Kreisbilder|Kategorie:Thresh Kreisbilder|Kategorie:Tristana Kreisbilder|Kategorie:Trundle Kreisbilder|Kategorie:Tryndamere Kreisbilder|Kategorie:Twisted Fate Kreisbilder|Kategorie:Twitch Kreisbilder|Kategorie:Udyr Kreisbilder|Kategorie:Urgot Kreisbilder|Kategorie:Varus Kreisbilder|Kategorie:Vayne Kreisbilder|Kategorie:Veigar Kreisbilder|Kategorie:Vel'Koz Kreisbilder|Kategorie:Vi Kreisbilder|Kategorie:Viktor Kreisbilder|Kategorie:Vladimir Kreisbilder|Kategorie:Volibear Kreisbilder|Kategorie:Warwick Kreisbilder|Kategorie:Wukong Kreisbilder|Kategorie:Xayah Kreisbilder|Kategorie:Xerath Kreisbilder|Kategorie:Xin Zhao Kreisbilder|Kategorie:Yasuo Kreisbilder|Kategorie:Yorick Kreisbilder|Kategorie:Yuumi Kreisbilder|Kategorie:Zac Kreisbilder|Kategorie:Zed Kreisbilder|Kategorie:Ziggs Kreisbilder|Kategorie:Zilean Kreisbilder|Kategorie:Zoe Kreisbilder|Kategorie:Zyra Kreisbilder")
    ]).await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token2 = String::from(o["edittoken"].as_str().unwrap());

    let json = client.request_json(&[
        ("action", "query"),
        ("format", "json"),
        ("prop", "info"),
        ("intoken", "edit"),
        ("titles", "Kategorie:Ahri Quadratbilder|Kategorie:Akali Quadratbilder|Kategorie:Alistar Quadratbilder|Kategorie:Amumu Quadratbilder|Kategorie:Anivia Quadratbilder|Kategorie:Annie Quadratbilder|Kategorie:Aphelios Quadratbilder|Kategorie:Ashe Quadratbilder|Kategorie:Aurelion Sol Quadratbilder|Kategorie:Azir Quadratbilder|Kategorie:Bard Quadratbilder|Kategorie:Blitzcrank Quadratbilder|Kategorie:Brand Quadratbilder|Kategorie:Braum Quadratbilder|Kategorie:Caitlyn Quadratbilder|Kategorie:Camille Quadratbilder|Kategorie:Cassiopeia Quadratbilder|Kategorie:Cho'Gath Quadratbilder|Kategorie:Corki Quadratbilder|Kategorie:Darius Quadratbilder|Kategorie:Diana Quadratbilder|Kategorie:Dr. Mundo Quadratbilder|Kategorie:Draven Quadratbilder|Kategorie:Ekko Quadratbilder|Kategorie:Elise Quadratbilder|Kategorie:Evelynn Quadratbilder|Kategorie:Ezreal Quadratbilder|Kategorie:Fiddlesticks Quadratbilder|Kategorie:Fiora Quadratbilder|Kategorie:Fizz Quadratbilder|Kategorie:Galio Quadratbilder|Kategorie:Gangplank Quadratbilder|Kategorie:Garen Quadratbilder|Kategorie:Gnar Quadratbilder|Kategorie:Gragas Quadratbilder|Kategorie:Graves Quadratbilder|Kategorie:Hecarim Quadratbilder|Kategorie:Heimerdinger Quadratbilder|Kategorie:Illaoi Quadratbilder|Kategorie:Irelia Quadratbilder|Kategorie:Ivern Quadratbilder|Kategorie:Janna Quadratbilder|Kategorie:Jarvan IV. Quadratbilder|Kategorie:Jax Quadratbilder|Kategorie:Jayce Quadratbilder|Kategorie:Jhin Quadratbilder|Kategorie:Jinx Quadratbilder|Kategorie:Kai'Sa Quadratbilder|Kategorie:Kalista Quadratbilder|Kategorie:Karma Quadratbilder|Kategorie:Karthus Quadratbilder|Kategorie:Kassadin Quadratbilder|Kategorie:Katarina Quadratbilder|Kategorie:Kayle Quadratbilder|Kategorie:Kayn Quadratbilder|Kategorie:Kennen Quadratbilder|Kategorie:Kha'Zix Quadratbilder|Kategorie:Kindred Quadratbilder|Kategorie:Kled Quadratbilder|Kategorie:Kog'Maw Quadratbilder|Kategorie:LeBlanc Quadratbilder|Kategorie:Lee Sin Quadratbilder|Kategorie:Leona Quadratbilder|Kategorie:Lissandra Quadratbilder|Kategorie:Lucian Quadratbilder|Kategorie:Lulu Quadratbilder|Kategorie:Lux Quadratbilder|Kategorie:Malphite Quadratbilder|Kategorie:Malzahar Quadratbilder|Kategorie:Maokai Quadratbilder|Kategorie:Master Yi Quadratbilder|Kategorie:Miss Fortune Quadratbilder|Kategorie:Mordekaiser Quadratbilder|Kategorie:Morgana Quadratbilder|Kategorie:Nami Quadratbilder|Kategorie:Nasus Quadratbilder|Kategorie:Nautilus Quadratbilder|Kategorie:Neeko Quadratbilder|Kategorie:Nidalee Quadratbilder|Kategorie:Nocturne Quadratbilder|Kategorie:Nunu & Willump Quadratbilder|Kategorie:Olaf Quadratbilder|Kategorie:Orianna Quadratbilder|Kategorie:Ornn Quadratbilder|Kategorie:Pantheon Quadratbilder|Kategorie:Poppy Quadratbilder|Kategorie:Pyke Quadratbilder|Kategorie:Qiyana Quadratbilder|Kategorie:Quinn Quadratbilder|Kategorie:Rakan Quadratbilder|Kategorie:Rammus Quadratbilder|Kategorie:Rek'Sai Quadratbilder|Kategorie:Renekton Quadratbilder|Kategorie:Rengar Quadratbilder|Kategorie:Riven Quadratbilder|Kategorie:Rumble Quadratbilder|Kategorie:Ryze Quadratbilder|Kategorie:Sejuani Quadratbilder|Kategorie:Senna Quadratbilder|Kategorie:Sett Quadratbilder|Kategorie:Shaco Quadratbilder|Kategorie:Shen Quadratbilder|Kategorie:Shyvana Quadratbilder|Kategorie:Singed Quadratbilder|Kategorie:Sion Quadratbilder|Kategorie:Sivir Quadratbilder|Kategorie:Skarner Quadratbilder|Kategorie:Sona Quadratbilder|Kategorie:Soraka Quadratbilder|Kategorie:Swain Quadratbilder|Kategorie:Sylas Quadratbilder|Kategorie:Syndra Quadratbilder|Kategorie:Tahm Kench Quadratbilder|Kategorie:Taliyah Quadratbilder|Kategorie:Talon Quadratbilder|Kategorie:Taric Quadratbilder|Kategorie:Teemo Quadratbilder|Kategorie:Thresh Quadratbilder|Kategorie:Tristana Quadratbilder|Kategorie:Trundle Quadratbilder|Kategorie:Tryndamere Quadratbilder|Kategorie:Twisted Fate Quadratbilder|Kategorie:Twitch Quadratbilder|Kategorie:Udyr Quadratbilder|Kategorie:Urgot Quadratbilder|Kategorie:Varus Quadratbilder|Kategorie:Vayne Quadratbilder|Kategorie:Veigar Quadratbilder|Kategorie:Vel'Koz Quadratbilder|Kategorie:Vi Quadratbilder|Kategorie:Viktor Quadratbilder|Kategorie:Vladimir Quadratbilder|Kategorie:Volibear Quadratbilder|Kategorie:Warwick Quadratbilder|Kategorie:Wukong Quadratbilder|Kategorie:Xayah Quadratbilder|Kategorie:Xerath Quadratbilder|Kategorie:Xin Zhao Quadratbilder|Kategorie:Yasuo Quadratbilder|Kategorie:Yorick Quadratbilder|Kategorie:Yuumi Quadratbilder|Kategorie:Zac Quadratbilder|Kategorie:Zed Quadratbilder|Kategorie:Ziggs Quadratbilder|Kategorie:Zilean Quadratbilder|Kategorie:Zoe Quadratbilder|Kategorie:Zyra Quadratbilder")
    ]).await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token3 = String::from(o["edittoken"].as_str().unwrap());

    let json = client.request_json(&[
        ("action", "query"),
        ("format", "json"),
        ("prop", "info"),
        ("intoken", "edit"),
        ("titles", "Kategorie:Ahri Ladebildschirmbilder|Kategorie:Akali Ladebildschirmbilder|Kategorie:Alistar Ladebildschirmbilder|Kategorie:Amumu Ladebildschirmbilder|Kategorie:Anivia Ladebildschirmbilder|Kategorie:Annie Ladebildschirmbilder|Kategorie:Aphelios Ladebildschirmbilder|Kategorie:Ashe Ladebildschirmbilder|Kategorie:Aurelion Sol Ladebildschirmbilder|Kategorie:Azir Ladebildschirmbilder|Kategorie:Bard Ladebildschirmbilder|Kategorie:Blitzcrank Ladebildschirmbilder|Kategorie:Brand Ladebildschirmbilder|Kategorie:Braum Ladebildschirmbilder|Kategorie:Caitlyn Ladebildschirmbilder|Kategorie:Camille Ladebildschirmbilder|Kategorie:Cassiopeia Ladebildschirmbilder|Kategorie:Cho'Gath Ladebildschirmbilder|Kategorie:Corki Ladebildschirmbilder|Kategorie:Darius Ladebildschirmbilder|Kategorie:Diana Ladebildschirmbilder|Kategorie:Dr. Mundo Ladebildschirmbilder|Kategorie:Draven Ladebildschirmbilder|Kategorie:Ekko Ladebildschirmbilder|Kategorie:Elise Ladebildschirmbilder|Kategorie:Evelynn Ladebildschirmbilder|Kategorie:Ezreal Ladebildschirmbilder|Kategorie:Fiddlesticks Ladebildschirmbilder|Kategorie:Fiora Ladebildschirmbilder|Kategorie:Fizz Ladebildschirmbilder|Kategorie:Galio Ladebildschirmbilder|Kategorie:Gangplank Ladebildschirmbilder|Kategorie:Garen Ladebildschirmbilder|Kategorie:Gnar Ladebildschirmbilder|Kategorie:Gragas Ladebildschirmbilder|Kategorie:Graves Ladebildschirmbilder|Kategorie:Hecarim Ladebildschirmbilder|Kategorie:Heimerdinger Ladebildschirmbilder|Kategorie:Illaoi Ladebildschirmbilder|Kategorie:Irelia Ladebildschirmbilder|Kategorie:Ivern Ladebildschirmbilder|Kategorie:Janna Ladebildschirmbilder|Kategorie:Jarvan IV. Ladebildschirmbilder|Kategorie:Jax Ladebildschirmbilder|Kategorie:Jayce Ladebildschirmbilder|Kategorie:Jhin Ladebildschirmbilder|Kategorie:Jinx Ladebildschirmbilder|Kategorie:Kai'Sa Ladebildschirmbilder|Kategorie:Kalista Ladebildschirmbilder|Kategorie:Karma Ladebildschirmbilder|Kategorie:Karthus Ladebildschirmbilder|Kategorie:Kassadin Ladebildschirmbilder|Kategorie:Katarina Ladebildschirmbilder|Kategorie:Kayle Ladebildschirmbilder|Kategorie:Kayn Ladebildschirmbilder|Kategorie:Kennen Ladebildschirmbilder|Kategorie:Kha'Zix Ladebildschirmbilder|Kategorie:Kindred Ladebildschirmbilder|Kategorie:Kled Ladebildschirmbilder|Kategorie:Kog'Maw Ladebildschirmbilder|Kategorie:LeBlanc Ladebildschirmbilder|Kategorie:Lee Sin Ladebildschirmbilder|Kategorie:Leona Ladebildschirmbilder|Kategorie:Lissandra Ladebildschirmbilder|Kategorie:Lucian Ladebildschirmbilder|Kategorie:Lulu Ladebildschirmbilder|Kategorie:Lux Ladebildschirmbilder|Kategorie:Malphite Ladebildschirmbilder|Kategorie:Malzahar Ladebildschirmbilder|Kategorie:Maokai Ladebildschirmbilder|Kategorie:Master Yi Ladebildschirmbilder|Kategorie:Miss Fortune Ladebildschirmbilder|Kategorie:Mordekaiser Ladebildschirmbilder|Kategorie:Morgana Ladebildschirmbilder|Kategorie:Nami Ladebildschirmbilder|Kategorie:Nasus Ladebildschirmbilder|Kategorie:Nautilus Ladebildschirmbilder|Kategorie:Neeko Ladebildschirmbilder|Kategorie:Nidalee Ladebildschirmbilder|Kategorie:Nocturne Ladebildschirmbilder|Kategorie:Nunu & Willump Ladebildschirmbilder|Kategorie:Olaf Ladebildschirmbilder|Kategorie:Orianna Ladebildschirmbilder|Kategorie:Ornn Ladebildschirmbilder|Kategorie:Pantheon Ladebildschirmbilder|Kategorie:Poppy Ladebildschirmbilder|Kategorie:Pyke Ladebildschirmbilder|Kategorie:Qiyana Ladebildschirmbilder|Kategorie:Quinn Ladebildschirmbilder|Kategorie:Rakan Ladebildschirmbilder|Kategorie:Rammus Ladebildschirmbilder|Kategorie:Rek'Sai Ladebildschirmbilder|Kategorie:Renekton Ladebildschirmbilder|Kategorie:Rengar Ladebildschirmbilder|Kategorie:Riven Ladebildschirmbilder|Kategorie:Rumble Ladebildschirmbilder|Kategorie:Ryze Ladebildschirmbilder|Kategorie:Sejuani Ladebildschirmbilder|Kategorie:Senna Ladebildschirmbilder|Kategorie:Sett Ladebildschirmbilder|Kategorie:Shaco Ladebildschirmbilder|Kategorie:Shen Ladebildschirmbilder|Kategorie:Shyvana Ladebildschirmbilder|Kategorie:Singed Ladebildschirmbilder|Kategorie:Sion Ladebildschirmbilder|Kategorie:Sivir Ladebildschirmbilder|Kategorie:Skarner Ladebildschirmbilder|Kategorie:Sona Ladebildschirmbilder|Kategorie:Soraka Ladebildschirmbilder|Kategorie:Swain Ladebildschirmbilder|Kategorie:Sylas Ladebildschirmbilder|Kategorie:Syndra Ladebildschirmbilder|Kategorie:Tahm Kench Ladebildschirmbilder|Kategorie:Taliyah Ladebildschirmbilder|Kategorie:Talon Ladebildschirmbilder|Kategorie:Taric Ladebildschirmbilder|Kategorie:Teemo Ladebildschirmbilder|Kategorie:Thresh Ladebildschirmbilder|Kategorie:Tristana Ladebildschirmbilder|Kategorie:Trundle Ladebildschirmbilder|Kategorie:Tryndamere Ladebildschirmbilder|Kategorie:Twisted Fate Ladebildschirmbilder|Kategorie:Twitch Ladebildschirmbilder|Kategorie:Udyr Ladebildschirmbilder|Kategorie:Urgot Ladebildschirmbilder|Kategorie:Varus Ladebildschirmbilder|Kategorie:Vayne Ladebildschirmbilder|Kategorie:Veigar Ladebildschirmbilder|Kategorie:Vel'Koz Ladebildschirmbilder|Kategorie:Vi Ladebildschirmbilder|Kategorie:Viktor Ladebildschirmbilder|Kategorie:Vladimir Ladebildschirmbilder|Kategorie:Volibear Ladebildschirmbilder|Kategorie:Warwick Ladebildschirmbilder|Kategorie:Wukong Ladebildschirmbilder|Kategorie:Xayah Ladebildschirmbilder|Kategorie:Xerath Ladebildschirmbilder|Kategorie:Xin Zhao Ladebildschirmbilder|Kategorie:Yasuo Ladebildschirmbilder|Kategorie:Yorick Ladebildschirmbilder|Kategorie:Yuumi Ladebildschirmbilder|Kategorie:Zac Ladebildschirmbilder|Kategorie:Zed Ladebildschirmbilder|Kategorie:Ziggs Ladebildschirmbilder|Kategorie:Zilean Ladebildschirmbilder|Kategorie:Zoe Ladebildschirmbilder|Kategorie:Zyra Ladebildschirmbilder"),
    ]).await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token4 = String::from(o["edittoken"].as_str().unwrap());

    for (_k, v) in champions {
        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated action"),
                ("bot", "1"),
                ("title", &format!("Kategorie:{} HD-Splasharts", v.name)),
                (
                    "text",
                    &format!(
                        "Diese Kategorie beinhaltet alle hochauflösenden Splasharts von {{{{ci|{}}}}}.
[[Kategorie:{}]][[Kategorie:Champion HD-Splasharts]]",
                        v.name, v.name
                    ),
                ),
                ("token", &edit_token1),
            ])
            .await?;

        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated action"),
                ("bot", "1"),
                ("title", &format!("Kategorie:{} Kreisbilder", v.name)),
                (
                    "text",
                    &format!(
                        "Diese Kategorie beinhaltet alle Kreisbilder von {{{{ci|{}}}}}.
[[Kategorie:{}]][[Kategorie:Champion Kreisbilder]]",
                        v.name, v.name
                    ),
                ),
                ("token", &edit_token2),
            ])
            .await?;

        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated action"),
                ("bot", "1"),
                ("title", &format!("Kategorie:{} Quadratbilder", v.name)),
                (
                    "text",
                    &format!(
                        "Diese Kategorie beinhaltet alle Quadratbilder von {{{{ci|{}}}}}.
[[Kategorie:{}]][[Kategorie:Champion Quadratbilder]]",
                        v.name, v.name
                    ),
                ),
                ("token", &edit_token3),
            ])
            .await?;

        client
            .request(&[
                ("action", "edit"),
                ("reason", "automated action"),
                ("bot", "1"),
                (
                    "title",
                    &format!("Kategorie:{} Ladebildschirmbilder", v.name),
                ),
                (
                    "text",
                    &format!(
                        "Diese Kategorie beinhaltet alle Ladebildschirmbilder von {{{{ci|{}}}}}.
[[Kategorie:{}]][[Kategorie:Champion Ladebildschirmbilder]]",
                        v.name, v.name
                    ),
                ),
                ("token", &edit_token4),
            ])
            .await?;

        tokio::time::delay_for(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
