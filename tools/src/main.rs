use structopt::StructOpt;
use serde_json::Value;

#[derive(StructOpt)]
struct Cli {
    func: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    if args.func == "delete" {
        delete_pages(content);
    }
}

fn delete_pages(content: String) {
    let botname = std::env::var("FANDOM_BOT_NAME").unwrap();
    let botpw = std::env::var("FANDOM_BOT_PASSWORD").unwrap();
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true).build().unwrap();
    let wiki_api_url = "https://leagueoflegends.fandom.com/de/api.php";

    let mut pages = "".to_owned();
    for line in content.lines() {
        pages.push_str(line);
        pages.push_str("|");
    }
    pages.pop();

    let res = client.post(wiki_api_url)
        .form(&[("action", "login"), ("format", "json"), ("lgname", &botname), ("lgpassword", &botpw)])
        .send().expect("Can't get login token").text().expect("Can't get login token from response body");

    let json: Value = serde_json::from_str(&res).unwrap();
    let token: String = String::from(json["login"]["token"].as_str().unwrap());


    let res = client.post(wiki_api_url)
        .form(&[("action", "login"), ("format", "json"), ("lgname", &botname), ("lgpassword", &botpw), ("lgtoken", &token)])
        .send().expect("Can't login with token");

    let res = client.post(wiki_api_url)
        .form(&[("action", "query"), ("format", "json"), ("prop", "info"), ("intoken", "delete"), ("titles", &pages)])
        .send().expect("Can't get delete token").text().expect("Can't get delete token from response body");

    let json: Value = serde_json::from_str(&res).unwrap();
    let (i, o) = json["query"]["pages"].as_object().unwrap().into_iter().next().unwrap();
    let delete_token = String::from(o["deletetoken"].as_str().unwrap());


    for line in content.lines() {
        client.post(wiki_api_url)
            .form(&[("action", "delete"), ("reason", "semi-automated action"), ("title", line), ("token", &delete_token)])
            .send().expect(&format!("Can't delete Page: {}", line));
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
}