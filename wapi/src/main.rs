use warp::{Filter, http::Uri};
use serde::{Serialize, Deserialize};
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
    pub splash_video_path: ::serde_json::Value,
    pub features_text: ::serde_json::Value,
    pub chroma_path: ::serde_json::Value,
    pub emblems: ::serde_json::Value,
    pub region_rarity_id: i64,
    pub rarity_gem_path: ::serde_json::Value,
    pub skin_lines: ::serde_json::Value,
    pub description: ::serde_json::Value,
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

#[tokio::main]
async fn main() {
    let link = warp::path!("wapi" / "skinlink" / String / String)
        .map(|champ: String, skin: String| {
            let champ = percent_encoding::percent_decode(champ.as_bytes()).decode_utf8().unwrap().replace("_", " ");
            let skin = percent_encoding::percent_decode(skin.as_bytes()).decode_utf8().unwrap().replace("_", " ").replace("~", "/");
            let (c, s) = get_skin(champ, skin);
            warp::redirect(format!("https://www.teemo.gg/model-viewer?skinid={}-{}&model-type=champions", c, s).parse::<Uri>().unwrap())
        });

    let update_champfile = warp::path!("wapi" / "update" / "champs" )
        .and_then(update_champs);

    let get_champfile = warp::path!("wapi" / "champs")
        .and(warp::fs::file("./champions.json"));

    warp::serve(link.or(update_champfile).or(get_champfile))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn update_champs() -> Result<impl warp::Reply, warp::Rejection> {
    let response = reqwest::get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/champion-summary.json")
        .await.expect("can't get champion-summary.json")
        .text().await.expect("can't get body from champion-summary response");

    let response2 = reqwest::get("https://raw.communitydragon.org/pbe/plugins/rcp-be-lol-game-data/global/de_de/v1/skins.json")
        .await.expect("can't get skins.json")
        .text().await.expect("can't get body from skins respone");

    let data: Vec<SummaryEntry> = serde_json::from_str(&response).expect("can't convert champion-summary to serde");
    let data2: HashMap<String, ChampSrc> = serde_json::from_str(&response2).expect("can't convert skins to serde");

    let mut champions = HashMap::new();

    for c in data.iter() {
        if c.id == -1 { continue };
        let temp = Champ {
            name: c.name.clone(),
            codename: c.alias.to_lowercase(),
            alias: c.alias.clone(),
            id: c.id,
            skins: Vec::new(),
        };
        champions.insert(temp.id, temp);
    }

    for (s, c) in data2.iter() {
        let skinpart: Vec<char> = s.chars().rev().take(3).collect();
        let skinid = format!("{}{}{}", skinpart[2], skinpart[1], skinpart[0]).parse::<i32>().unwrap();
        let champpart: Vec<char> = s.chars().take(c.id.to_string().len()-3).collect();
        let champstring: String = champpart.into_iter().collect();
        let champid: i32 = champstring.parse::<i32>().unwrap();

        let temp = Skin {
            id: skinid,
            id_long: s.parse().unwrap(),
            name: c.name.clone(),
        };

        champions.get_mut(&champid).unwrap().skins.push(temp);
    }

    ::serde_json::to_writer(&File::create("champions.json").expect("Can't create champions.json file"), &champions).expect("Failed to save champions.json to disk");

    Ok("Success")
}

fn get_skin(champ: String, skin: String) -> (String, i32) {
    let json: HashMap<i32, Champ> = ::serde_json::from_reader(&File::open("champions.json").expect("Can't read champions.json")).expect("Can't read champions.json");
    for (_i, c) in json.iter() {
        if c.name == champ {
            for s in c.skins.iter() {
                if s.name == skin {
                    return (c.codename.clone(), s.id);
                }
            }
        }
    }
    (champ.to_lowercase(), 0)
}