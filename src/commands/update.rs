use futures::{future::TryFutureExt, try_join};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSrc {
    pub id: i32,
    pub is_base: bool,
    pub name: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub skin_type: String,
    pub rarity: String,
    pub is_legacy: bool,
    pub splash_video_path: Value,
    pub features_text: Value,
    pub chroma_path: Value,
    pub emblems: Value,
    pub region_rarity_id: i64,
    pub rarity_gem_path: Value,
    pub skin_lines: Value,
    pub description: Value,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Rotations {
    free_champion_ids: Vec<i32>,
    free_champion_ids_for_new_players: Vec<i32>,
    max_new_player_level: i32,
}

pub async fn champs() -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn rotation() -> Result<(), Box<dyn std::error::Error>> {
    let riot_api_url = "https://euw1.api.riotgames.com/lol/platform/v3/champion-rotations?api_key="
        .to_owned()
        + &std::env::var("RIOT_API_KEY")?;
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";
    let curr_date = chrono::Utc::today();
    let dates = [
        rename_m(
            (curr_date - chrono::Duration::days(14))
                .format("%-d. %B %Y")
                .to_string(),
        ),
        rename_m(
            (curr_date - chrono::Duration::days(7))
                .format("%-d. %B %Y")
                .to_string(),
        ),
        rename_m(curr_date.format("%-d. %B %Y").to_string()),
        rename_m(
            (curr_date + chrono::Duration::days(7))
                .format("%-d. %B %Y")
                .to_string(),
        ),
    ];
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    crate::helpers::wiki::wiki_login(&client).await?;

    let champions: HashMap<i32, Champ> = client
        .get("https://fabianlars.de/wapi/champs")
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
    let new_players: String = new_players.iter().map(|x| "|".to_owned() + x).collect();

    let mut history: Vec<String> = serde_json::from_reader(&std::fs::File::open("history.json")?)?;
    history.pop();
    history.insert(0, rotation.iter().map(|x| "|".to_owned() + x).collect());

    serde_json::to_writer(&std::fs::File::create("history.json")?, &history)?;

    let res = client
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
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&res)?;
    let (_i, o) = json["query"]["pages"]
        .as_object()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let edit_token = String::from(o["edittoken"].as_str().unwrap());

    let template = format!(
        r#"
<div style="text-align:center; font-size: 125%; font-weight:bold; margin: 2px 0 0;">[[Kostenlose Championrotation]]</div><div style="text-align:left; font-size: 80%; font-weight:bold; margin: 2px 0 0;">[[Vorlage:Aktuelle Championrotation|Bearbeiten]]</div>
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
<p style="text-align: center; margin: 0 15%;">''Alle Zufällig''-Spiele erlauben es Spielern, Champions aus den letzten beiden Championrotationen sowie aus der aktuellen zu rollen. Dopplungen erhohen hierbei nicht die Wahrscheinlichkeit, den Champion zu ziehen.</p>
{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = {}
|dateto           = {}
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
{}}}}}

{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = {}
|dateto           = {}
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen!-->
{}}}}}

{{{{Aktuelle Championrotation/var
|specialweek      = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialstartdate = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|specialenddate   = <!-- Nur für Sonderfälle, sonst leer lassen! -->
|datefrom         = {}
|dateto           = {}
|lastchecked      = <!-- Nur für die Rotation neuer Accounts, sonst leer lassen! -->
{}}}}}


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
</tabber><noinclude>{{Dokumentation}}<noinclude>
    "#,
        history[0],
        dates[0],
        dates[1],
        history[2],
        dates[1],
        dates[2],
        history[1],
        dates[2],
        dates[3],
        history[0],
        dates[2],
        new_players
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
