use std::{collections::HashMap, error::Error, fs::File};

use futures::{future::TryFutureExt, try_join};
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::{props::*, wiki};

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

pub async fn champs() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let fut1 = async {
        let response = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champion-summary.json").send().await?.json::<Vec<SummaryEntry>>().await?;
        Ok::<Vec<SummaryEntry>, reqwest::Error>(response)
    }.map_err(|_e| "Can't get or convert champion-summary.json".to_string());
    let fut2 = async {
        let response = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await?.json::<HashMap<String, ChampSrc>>().await?;
        Ok::<HashMap<String, ChampSrc>, reqwest::Error>(response)
    }.map_err(|_e| "Can't get or convert skins.json".to_string());

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

    ::serde_json::to_writer(&File::create("champions.json")?, &champions)?;

    Ok(())
}

pub async fn discounts(props: Props) -> Result<(), Box<dyn Error>> {
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";
    let lockfile = std::fs::read_to_string(props.path.file_path()).unwrap();
    // 0: "LeagueClient", 1: PID, 2: Port, 3: Auth, 4: Protocol
    let contents = lockfile.split(':').collect::<Vec<_>>();
    let port = contents[2];
    let auth = base64::encode(format!("riot:{}", contents[3]).as_bytes());
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Basic {}", auth).parse().unwrap());
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .cookie_store(true)
        .build()?;
    let json: Vec<StoreChamp> = client
        .get(&format!(
            "https://127.0.0.1:{}/lol-store/v1/catalog?inventoryType=[\"CHAMPION\",\"CHAMPION_SKIN\"]",
            port
        ))
        .send()
        .await?
        .json()
        .await?;

    let champions_wapi: HashMap<i32, Champ> = client
        .get("https://fabianlars.de/wapi/champion")
        .send()
        .await?
        .json()
        .await?;

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

    wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    let json: Value = client
        .get(reqwest::Url::parse_with_params(
            wiki_api_url,
            &[
                ("action", "query"),
                ("format", "json"),
                ("prop", "info"),
                ("intoken", "edit"),
                ("titles", "Vorlage:Aktuelle_Angebote"),
            ],
        )?)
        .send()
        .await?
        .json()
        .await?;

    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    client
        .post(wiki_api_url)
        .form(&[
            ("action", "edit"),
            ("reason", "Nicht ganz so automatische Aktion"),
            ("bot", "1"),
            ("title", "Vorlage:Aktuelle_Angebote"),
            ("text", &full_template),
            ("token", &edit_token),
        ])
        .send()
        .await?;

    Ok(())
}

#[cfg(feature = "riot-api")]
pub async fn rotation(props: Props) -> Result<(), Box<dyn Error>> {
    let riot_api_url = "https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key="
        .to_owned()
        + &std::env::var("RIOT_API_KEY")?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let curr_date = rename_m(chrono::Utc::today().format("%-d. %B %Y").to_string());

    wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    let champions: HashMap<i32, Champ> = client
        .get("https://fabianlars.de/wapi/champion")
        .send()
        .await?
        .json()
        .await?;

    let res = client.get(&riot_api_url).send().await?.text().await?;
    let rotations: Rotations = serde_json::from_str(&res)?;

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

    let json: Value = client
        .get(reqwest::Url::parse_with_params(
            wiki_api_url,
            &[
                ("action", "query"),
                ("format", "json"),
                ("prop", "info"),
                ("intoken", "edit"),
                ("titles", "Vorlage:Aktuelle_Championrotation"),
            ],
        )?)
        .send()
        .await?
        .json()
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
|Aatrox|Ahri|Akali|Amumu|Annie|Ashe|Brand|Braum|Caitlyn|Cho'Gath|Darius|Draven|Ekko|Ezreal|Fiora|Fizz|Garen|Graves|Irelia|Janna|Jarvan IV|Jax|Jhin|Jinx|Karma|Karthus|Katarina|Kayle|Kha'Zix|LeBlanc|Lee Sin|Leona|Lucian|Lulu|Lux|Malphite|Maokai|Master Yi|Miss Fortune|Mordekaiser|Morgana|Nautilus|Nidalee|Pantheon|Pyke|Quinn|Renekton|Riven|Ryze|Sivir|Sona|Soraka|Thresh|Tristana|Tryndamere|Twisted Fate|Twitch|Varus|Vayne|Veigar|Vel'Koz|Vladimir|Wukong|Xayah|Zed}}}}
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
        .post(wiki_api_url)
        .form(&[
            ("action", "edit"),
            ("reason", "automated action"),
            ("bot", "1"),
            ("title", "Vorlage:Aktuelle_Championrotation"),
            ("text", &template),
            ("token", &edit_token),
        ])
        .send()
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

pub(crate) async fn set(props: Props) -> Result<(), Box<dyn Error>> {
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let mut edit_token = String::new();
    let mut skin: String = String::new();
    let mut set: String = String::new();
    let mut universe: String = String::new();
    let mut icons: String = String::new();
    let mut iconsets: String = String::new();
    let mut champion: String = String::new();
    let mut tft: String = String::new();

    wiki::wiki_login(&client, props.loginname, props.loginpassword).await?;

    let fut_token = async {
        let json: Value = client
            .get(reqwest::Url::parse_with_params(
                wiki_api_url,
                &[
                    ("action", "query"),
                    ("format", "json"),
                    ("prop", "info"),
                    ("intoken", "edit"),
                    ("titles", "Vorlage:Set/skins.json|Vorlage:Set/sets.json|Vorlage:Set/universes.json|Vorlage:Set/icons.json|Vorlage:Set/iconsets.json|Vorlage:Set/champion.json|Vorlage:Set/TFT.json"),
                ],
            ).unwrap())
            .send()
            .await?
            .json()
            .await?;

        let (_i, o) = json["query"]["pages"]
            .as_object()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        edit_token = String::from(o["edittoken"].as_str().unwrap());
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get skins.json".to_string());
    let fut_skin = async {
        skin = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get skins.json".to_string());
    let fut_set = async {
        set = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skinlines.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get skinlines.json".to_string());
    let fut_universe = async {
        universe = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/universes.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_icons = async {
        icons = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icons.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_iconsets = async {
        iconsets = client.get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/summoner-icon-sets.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_champion = async {
        let res: Value = client.get("https://ddragon.leagueoflegends.com/api/versions.json").send().await?.json().await?;
        let patch_id = res.get(0).unwrap().as_str().unwrap();
        champion = client.get(&format!("http://ddragon.leagueoflegends.com/cdn/{}/data/de_DE/champion.json", patch_id)).send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_tft = async {
        tft = client.get("http://raw.communitydragon.org/latest/cdragon/tft/de_de.json").send().await?.text().await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());

    try_join!(fut_token, fut_skin, fut_set, fut_universe, fut_icons, fut_iconsets, fut_champion, fut_tft)?;


    let fut_skin = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/skins.json"),
                ("text", &skin),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get skins.json".to_string());
    let fut_set = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/sets.json"),
                ("text", &set),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get skinlines.json".to_string());
    let fut_universe = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/universes.json"),
                ("text", &universe),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_icons = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/icons.json"),
                ("text", &icons),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_iconsets = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/iconsets.json"),
                ("text", &iconsets),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_champion = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/champion.json"),
                ("text", &champion),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());
    let fut_tft = async {
        client
            .post(wiki_api_url)
            .form(&[
                ("action", "edit"),
                ("reason", "automated update"),
                ("bot", "1"),
                ("title", "Vorlage:Set/TFT.json"),
                ("text", &tft),
                ("token", &edit_token),
            ])
            .send()
            .await?;
        Ok::<(), reqwest::Error>(())
    }.map_err(|_e| "Can't get universes.json".to_string());

    try_join!(fut_skin, fut_set, fut_universe, fut_icons, fut_iconsets, fut_champion, fut_tft)?;

    Ok(())
}