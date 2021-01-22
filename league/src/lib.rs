#![forbid(unsafe_code)]

use std::collections::HashMap;

use anyhow::{anyhow, Error, Result};
use futures::{join, prelude::*, try_join};
use regex::Regex;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};
use reqwest::Client;
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

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

#[derive(Debug, Deserialize)]
struct Parse {
    title: String,
    wikitext: String,
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
    let lockfile = std::fs::read_to_string(path.file_path()?).unwrap();
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
        .client()
        .get("https://api.fabianlars.de/v1/wiki/champions")
        .send()
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

    log::info!(
        "{:?}",
        client
            .post_into_text(&[
                ("action", "edit"),
                ("summary", "Nicht ganz so automatische Aktion"),
                ("bot", "1"),
                ("title", "Vorlage:Aktuelle_Angebote"),
                ("text", &full_template),
            ])
            .await?
    );

    Ok(())
}

#[cfg(feature = "riot-api")]
pub async fn rotation<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let client = client.as_ref();
    let riot_api_url = format!(
        "https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key={}",
        &std::env::var("RIOT_API_KEY")?
    );

    let curr_date = chrono::Utc::today()
        .format_localized("%-d. %B %Y", chrono::Locale::de_DE)
        .to_string();

    let champions: HashMap<i32, Champ> = client
        .client()
        .get("https://api.fabianlars.de/v1/wiki/champions")
        .send()
        .await?
        .json()
        .await?;

    let rotations: Rotations = client
        .client()
        .get(&riot_api_url)
        .send()
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

    let template = format!(
        r#"{{{{Kopfzeile|[[Kostenlose Championrotation]]}}}}
<tabber>Klassisch=<h3 style="display:none">Klassisch</h3>
{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|dateto           = <!-- Nur für die ARAM-Rotation verwendet, sonst leer lassen! -->
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
{}}}}}
|-|ARAM=<h3 style="display:none">ARAM</h3>
<p style="text-align: center; margin: 0 15%;">In ''Alle Zufällig''-Spielen sind zusätzlich zur normalen Rotation folgende 65 Champions immer möglich:</p>
{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|dateto           = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|lastchecked      = {}
|Aatrox|Ahri|Akali|Amumu|Annie|Ashe|Brand|Braum|Caitlyn|Cho'Gath|Darius|Draven|Ekko|Ezreal|Fiora|Fizz|Garen|Graves|Irelia|Janna|Jarvan IV.|Jax|Jhin|Jinx|Karma|Karthus|Katarina|Kayle|Kha'Zix|LeBlanc|Lee Sin|Leona|Lucian|Lulu|Lux|Malphite|Maokai|Master Yi|Miss Fortune|Mordekaiser|Morgana|Nautilus|Nidalee|Pantheon|Pyke|Quinn|Renekton|Riven|Ryze|Sivir|Sona|Soraka|Thresh|Tristana|Tryndamere|Twisted Fate|Twitch|Varus|Vayne|Veigar|Vel'Koz|Vladimir|Wukong|Xayah|Zed}}}}
|-|Neue Accounts=<h3 style="display:none">Neue Accounts</h3>
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
        .post(&[
            ("action", "edit"),
            ("summary", "automated action"),
            ("bot", "1"),
            ("title", "Vorlage:Aktuelle_Championrotation"),
            ("text", &template),
        ])
        .await?;

    Ok(())
}

pub async fn set<C: AsRef<WikiClient>>(client: C) -> Result<()> {
    let mut skin: String = String::new();
    let mut set: String = String::new();
    let mut universe: String = String::new();
    let mut icons: String = String::new();
    let mut iconsets: String = String::new();
    let mut champion: String = String::new();
    let mut tft: String = String::new();
    let client = client.as_ref();
    let ext_client = client.client();
    let lua_regex = Regex::new(r#""(?P<k>\w+)":"#)?;

    let convert = |x: String| {
        let lua: String = x
            .chars()
            .map(|s| match s {
                '[' => '{',
                ']' => '}',
                _ => s,
            })
            .collect();

        let lua = lua.replace("null,", "nil,");
        let lua = lua_regex.replace_all(&lua, "[\"$k\"]=");

        format!("return {}", lua)
    };

    let fut_skin = async {
        skin = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await?.text().await?.replace(" ", " ").replace("Hexerei-Miss Fortune \"", "Hexerei-Miss Fortune\"");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get skins.json"));
    let fut_set = async {
        set = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skinlines.json").send().await?.text().await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get skinlines.json"));
    let fut_universe = async {
        universe = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/universes.json").send().await?.text().await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_icons = async {
        icons = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icons.json").send().await?.text().await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_iconsets = async {
        iconsets = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icon-sets.json").send().await?.text().await?.replace(" ", " ");
        Ok::<(), Error>(())
    }.map_err(|_| anyhow!("Can't get universes.json"));
    let fut_champion = async {
        let patches: Vec<String> = ext_client
            .get("https://ddragon.leagueoflegends.com/api/versions.json")
            .send()
            .await?
            .json()
            .await?;
        champion = ext_client
            .get(&format!(
                "http://ddragon.leagueoflegends.com/cdn/{}/data/de_DE/champion.json",
                patches[0]
            ))
            .send()
            .await?
            .text()
            .await?
            .replace(" ", " ");
        Ok::<(), Error>(())
    }
    .map_err(|_| anyhow!("Can't get universes.json"));
    let fut_tft = async {
        tft = ext_client
            .get("http://raw.communitydragon.org/latest/cdragon/tft/de_de.json")
            .send()
            .await
            .expect("Can't get universes.json")
            .text()
            .await?
            .replace(" ", " ");
        Ok::<(), Error>(())
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

    let fut_skin = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/skins.json"),
                ("text", &skin),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit skins.json"));
    let fut_set = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/sets.json"),
                ("text", &set),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit skinlines.json"));
    let fut_universe = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/universes.json"),
                ("text", &universe),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.json"));
    let fut_icons = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/icons.json"),
                ("text", &icons),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.json"));
    let fut_iconsets = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/iconsets.json"),
                ("text", &iconsets),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.json"));
    let fut_champion = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/champion.json"),
                ("text", &champion),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.json"));
    let fut_tft = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/TFT.json"),
                ("text", &tft),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.json"));

    try_join!(
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
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/skins.src"),
                ("text", &convert(skin)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit skins.src"));
    let fut_set = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/sets.src"),
                ("text", &convert(set)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit skinlines.src"));
    let fut_universe = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/universes.src"),
                ("text", &convert(universe)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.src"));
    let fut_icons = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/icons.src"),
                ("text", &convert(icons)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.src"));
    let fut_iconsets = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/iconsets.json"),
                ("text", &convert(iconsets)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.src"));
    let fut_champion = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/champion.src"),
                ("text", &convert(champion)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.src"));
    let fut_tft = async {
        client
            .post(&[
                ("action", "edit"),
                ("summary", "automated update"),
                ("bot", "1"),
                ("title", "Modul:Set/TFT.src"),
                ("text", &convert(tft)),
            ])
            .await
    }
    .map_err(|_| anyhow!("Can't edit universes.src"));

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

    let mut positions: Vec<(String, String)> = Vec::new();
    let mut new_champdata: Vec<String> = Vec::new();

    let (resp, resp2) = join!(
        client.client().get(opgg).send(),
        client.get_into_json(&[
            ("action", "parse"),
            ("page", "Module:ChampionData/data"),
            ("prop", "wikitext"),
        ])
    );
    let resp = resp?.text().await?;
    let document = Document::from(resp.as_str());

    let champdata: HashMap<String, Parse> = resp2?;
    let champdata = &champdata.get("parse").unwrap().wikitext;
    let champdata_regex = Regex::new("(?m)\\[\"op_positions\"] *= .+,$")?;
    let champdata_iter = champdata_regex.split(champdata);

    for node in document.find(Class("champion-index__champion-item")) {
        let mut temp_positions: Vec<String> = Vec::new();

        let name = node
            .find(Class("champion-index__champion-item__name"))
            .next()
            .expect("Panicking is alright")
            .text();

        let classes = node.attr("class").expect("Can't get classes of node");

        if classes.contains("champion-index__champion-item--TOP") {
            temp_positions.push("\"Oben\"".to_string());
        }
        if classes.contains("champion-index__champion-item--MID") {
            temp_positions.push("\"Mitte\"".to_string());
        }
        if classes.contains("champion-index__champion-item--ADC") {
            temp_positions.push("\"Unten\"".to_string());
        }
        if classes.contains("champion-index__champion-item--SUPPORT") {
            temp_positions.push("\"Unterstützer\"".to_string());
        }
        if classes.contains("champion-index__champion-item--JUNGLE") {
            temp_positions.push("\"Dschungel\"".to_string());
        }
        positions.push((
            if name.contains("Jarvan") {
                "Jarvan IV.".to_string()
            } else {
                name
            },
            temp_positions.join(", "),
        ));
    }

    for block in champdata_iter {
        let mut action_done = false;
        new_champdata.push(block.to_string());
        for (champ, pos) in &positions {
            if block.contains(&format!("[\"{}\"]", champ)) {
                new_champdata.push(format!("[\"op_positions\"] = {{{}}},", pos));
                action_done = true;
                break;
            }
        }
        if !action_done {
            new_champdata.push("[\"op_positions\"] = {},".to_string());
        }
    }
    new_champdata.pop();

    client
        .post(&[
            ("action", "edit"),
            ("summary", "automated action"),
            ("bot", "1"),
            ("title", "Module:ChampionData/data"),
            ("text", &new_champdata.concat()),
        ])
        .await?;

    Ok(())
}
