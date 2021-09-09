#![forbid(unsafe_code)]

use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Error, Result};
use futures_util::future::join;
use regex::Regex;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};
use reqwest::Client as ReqwestClient;
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use mw_tools::Client;

type Ignore = serde::de::IgnoredAny;

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChampSrc {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SummaryEntry {
    id: i32,
    name: String,
    alias: String,
    //square_portrait_path: String,
    //roles: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Champ {
    name: String,
    codename: String,
    alias: String,
    id: i32,
    skins: Vec<Skin>,
}

#[derive(Deserialize, Serialize)]
struct Skin {
    id: i32,
    id_long: i32,
    name: String,
}

#[cfg(feature = "riot-api")]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Rotations {
    free_champion_ids: Vec<i32>,
    free_champion_ids_for_new_players: Vec<i32>,
    //max_new_player_level: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoreChamp {
    inventory_type: String,
    item_id: i32,
    item_requirements: Option<Vec<ItemReq>>,
    sale: Option<Sale>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemReq {
    //inventory_type: String,
    item_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sale {
    end_date: String,
    prices: Vec<Price>,
    start_date: String,
}

#[derive(Deserialize)]
struct Price {
    //cost: i32,
    currency: String,
    discount: f32,
}

struct Angebot {
    champ: String,
    skin: Option<String>,
    discount: String,
}

#[derive(Deserialize)]
struct Parse {
    //title: String,
    wikitext: String,
}

pub async fn champs() -> Result<()> {
    let client = ReqwestClient::new();

    let fut1 = async {
        let response: Vec<SummaryEntry> = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champion-summary.json").send().await?.json().await?;
        Ok::<Vec<SummaryEntry>, Error>(response)
    };
    let fut2 = async {
        let response: HashMap<String, ChampSrc> = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await?.json().await?;
        Ok::<HashMap<String, ChampSrc>, Error>(response)
    };

    let (summary, skins) = join(fut1, fut2).await;

    let mut champions = HashMap::new();

    for c in &summary? {
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

    for (s, c) in &skins? {
        let skinpart: Vec<char> = s.chars().rev().take(3).collect();
        let skinid = format!("{}{}{}", skinpart[2], skinpart[1], skinpart[0]).parse::<i32>()?;
        let champstring: String = s.chars().take(c.id.to_string().len() - 3).collect();
        let champid: i32 = champstring.parse::<i32>()?;

        let temp = Skin {
            id: skinid,
            id_long: s.parse()?,
            name: c.name.clone(),
        };

        if let Some(champ) = champions.get_mut(&champid) {
            champ.skins.push(temp);
        }
    }

    File::create("champions.json")
        .await?
        .write_all(serde_json::to_string(&champions)?.as_bytes())
        .await?;

    Ok(())
}

pub async fn discounts<C: AsRef<Client>>(client: C, path: PathBuf) -> Result<()> {
    let client = client.as_ref();
    let lockfile = std::fs::read_to_string(path)?;
    // 0: "LeagueClient", 1: PID, 2: Port, 3: Auth, 4: Protocol
    let contents = lockfile.split(':').collect::<Vec<_>>();
    let port = contents[2];
    let auth = base64::encode(format!("riot:{}", contents[3]).as_bytes());
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    headers.insert(AUTHORIZATION, format!("Basic {}", auth).parse()?);
    let unsafe_client = ReqwestClient::builder()
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
        .get("https://api.fabianlars.de/v1/lol/champions")
        .send()
        .await?
        .json()
        .await
        .map_err(|e| anyhow!("Can't convert champion json to hashmap: {}", e))?;

    let mut champs: Vec<Angebot> = Vec::new();
    let mut skins: Vec<Angebot> = Vec::new();
    let mut start_date = String::new();
    let mut end_date = String::new();

    for entry in json {
        let sale = match entry.sale {
            Some(s) => s,
            None => continue,
        };

        match entry.inventory_type.as_str() {
            "CHAMPION" => {
                let mut discount = "discount_error".to_string();
                for p in sale.prices {
                    if p.currency == "RP" {
                        if let Some(s) = format!("{:.2}", p.discount).split('.').last() {
                            discount = s.to_string();
                        }
                    }
                }

                start_date = sale.start_date.clone();
                end_date = sale.end_date.clone();

                champs.push(Angebot {
                    champ: champions_wapi[&entry.item_id].name.clone(),
                    skin: None,
                    discount,
                });
            }
            "CHAMPION_SKIN" => {
                let champ_id: i32 = match entry.item_requirements {
                    Some(ir) => ir[0].item_id,
                    None => -1,
                };

                let mut skin = "skin_error".to_string();
                let mut discount = "discount_error".to_string();

                for s in &champions_wapi[&champ_id].skins {
                    if s.id_long == entry.item_id {
                        skin = s.name.to_string();
                    }
                }

                for p in sale.prices {
                    if p.currency == "RP" {
                        if let Some(s) = format!("{:.2}", p.discount).split('.').last() {
                            discount = s.to_string();
                        }
                    }
                }

                skins.push(Angebot {
                    champ: champions_wapi[&champ_id].name.clone(),
                    skin: Some(skin),
                    discount,
                });
            }
            _ => {
                continue;
            }
        }
    }

    champs.sort_by(|a, b| a.champ.cmp(&b.champ));
    skins.sort_by(|a, b| a.champ.cmp(&b.champ));

    let mut angebote: String = "".to_string();

    if let Some(date) = start_date.split('T').next() {
        let date_vec: Vec<_> = date.split('-').collect();
        start_date = format!("{}.{}.{}", date_vec[2], date_vec[1], date_vec[0]);
    }
    if let Some(date) = end_date.split('T').next() {
        let date_vec: Vec<_> = date.split('-').collect();
        end_date = format!("{}.{}.{}", date_vec[2], date_vec[1], date_vec[0]);
    }

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
        ));
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
            // unwrapping here is save
            s.skin.as_ref().unwrap(),
            s.discount
        ));
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

    let res: mw_tools::response::edit::Edit = client
        .post(&[
            ("action", "edit"),
            ("summary", "Nicht ganz so automatische Aktion"),
            ("bot", ""),
            ("title", "Vorlage:Aktuelle_Angebote"),
            ("text", &full_template),
        ])
        .await?;

    log::info!("{:?}", res);

    Ok(())
}

#[cfg(feature = "riot-api")]
pub async fn rotation<C: AsRef<Client>>(client: C) -> Result<()> {
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
        .get("https://api.fabianlars.de/v1/lol/champions")
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
    let rotation_str: String = rotation.iter().map(|x| "|".to_owned() + x).collect();
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
        rotation_str, curr_date, curr_date, new_players
    );

    let sql = format!(
        "INSERT INTO rotations(start_date, end_date, champions) VALUES ('{}', '{}', ARRAY[{}]) ON CONFLICT DO NOTHING;",
        chrono::Utc::today()
            .format_localized("%Y-%m-%d", chrono::Locale::de_DE)
            .to_string(),
        std::ops::Add::add(chrono::Utc::today(), chrono::Duration::days(7))
            .format_localized("%Y-%m-%d", chrono::Locale::de_DE)
            .to_string(),
        rotation
            .iter()
            .map(|x| format!("'{}'", x.replace("'", "''")))
            .collect::<Vec<String>>()
            .join(",")
    );

    File::create("new_rotation.sql")
        .await?
        .write_all(sql.as_bytes())
        .await?;

    client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated action"),
            ("bot", ""),
            ("title", "Vorlage:Aktuelle_Championrotation"),
            ("text", &template),
        ])
        .await?;

    Ok(())
}

pub async fn set<C: AsRef<Client>>(client: C) -> Result<()> {
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

    let skin = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await?.text().await?.replace(" ", " ").replace("Hexerei-Miss Fortune \"", "Hexerei-Miss Fortune\"");

    let set = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skinlines.json").send().await?.text().await?.replace(" ", " ");

    let universe = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/universes.json").send().await?.text().await?.replace(" ", " ");

    let icons = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icons.json").send().await?.text().await?.replace(" ", " ");

    let iconsets = ext_client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icon-sets.json").send().await?.text().await?.replace(" ", " ");

    let patches: Vec<String> = ext_client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()
        .await?
        .json()
        .await?;
    let champion = ext_client
        .get(&format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/de_DE/champion.json",
            patches[0]
        ))
        .send()
        .await?
        .text()
        .await?
        .replace(" ", " ");

    let tft = ext_client
        .get("http://raw.communitydragon.org/latest/cdragon/tft/de_de.json")
        .send()
        .await
        .expect("Can't get universes.json")
        .text()
        .await?
        .replace(" ", " ");

    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/skins.json"),
            ("text", &skin),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/sets.json"),
            ("text", &set),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/universes.json"),
            ("text", &universe),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/icons.json"),
            ("text", &icons),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/iconsets.json"),
            ("text", &iconsets),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/champion.json"),
            ("text", &champion),
        ])
        .await?;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Vorlage:Set/TFT.json"),
            ("text", &tft),
        ])
        .await;

    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/skins.src"),
            ("text", &convert(skin)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/sets.src"),
            ("text", &convert(set)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/universes.src"),
            ("text", &convert(universe)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/icons.src"),
            ("text", &convert(icons)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/iconsets.json"),
            ("text", &convert(iconsets)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/champion.src"),
            ("text", &convert(champion)),
        ])
        .await;
    let _ = client
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated update"),
            ("bot", ""),
            ("title", "Modul:Set/TFT.src"),
            ("text", &convert(tft)),
        ])
        .await;

    Ok(())
}

pub async fn positions<C: AsRef<Client>>(client: C) -> Result<()> {
    let client = client.as_ref();
    let opgg = "https://euw.op.gg/champion/statistics";

    let mut positions: Vec<(String, String)> = Vec::new();
    let mut new_champdata: Vec<String> = Vec::new();

    let (resp, resp2) = join(
        client.client().get(opgg).send(),
        client.get(&[
            ("action", "parse"),
            ("page", "Module:Champion/data"),
            ("prop", "wikitext"),
        ]),
    )
    .await;
    let resp = resp?.text().await?;
    let document = Document::from(resp.as_str());

    let champdata: HashMap<String, Parse> = resp2?;
    let champdata = match &champdata.get("parse") {
        Some(parse) => &parse.wikitext,
        None => return Err(anyhow!("Response doesn't contain requested wikitext")),
    };
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
        .post::<Ignore>(&[
            ("action", "edit"),
            ("summary", "automated action"),
            ("bot", ""),
            ("title", "Module:Champion/data"),
            ("text", &new_champdata.concat()),
        ])
        .await?;

    Ok(())
}
