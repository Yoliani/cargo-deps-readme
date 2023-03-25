extern crate env_logger;
extern crate log;

use std::fs::File;

use deps_readme::entities::Crate;
use deps_readme::entities::CrateApiEntity;
use log::info;
use reqwest::Client as ReqwestClient;
use reqwest::header;
use reqwest::header::HeaderMap;
use std::io::prelude::*;


lazy_static::lazy_static! {
    static ref CRATE_API: String = String::from("https://crates.io/api/v1/crates?page=1&per_page=10&q=");
}



#[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting up");
    let client_req = ReqwestClient::new();
    let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("my-user-agent (my-contact@domain.com)")?,
        );

    let mut file = File::open("Cargo.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    info!("{}{}", "contents: ", contents);
    let mut deps: Vec<String> = Vec::new();
    for  (idx, c) in contents.lines().enumerate() {
        if c.contains("[dependencies]") {
            info!("{}{}", "idx: ", idx);
            for (idx2, c2) in contents.lines().enumerate() {
                if idx2 > idx && c2.contains("=")  {
                        let dep = c2.split("=").collect::<Vec<&str>>();
                        deps.push(dep[0].to_string().split(" ").collect::<Vec<&str>>()[0].to_string());
                }
            }
        }

    }


    let mut file_output = File::create("DEPENDENCIES.md")
   .expect("Error encountered while creating file!");

    let mut file_text: Vec<String> = Vec::new();
    file_text.push(String::from("# Dependencies\n\n"));
    for dep in deps {
        let response = get_one_crate_api_entity(&client_req, &headers, &dep).await?;
        if response.is_none() {
            continue;
        }
        let response = response.unwrap();
        let link = if let Some(li) = response.repository {
            li
        }else {
            format!("https://crates.io/crates/{}", response.id)
        };

        let text = format!("* [{}]({})\n", response.id, link);
        file_text.push(text.to_owned());
    }

    let _ = file_output.write_all(file_text.join("").as_bytes());

    Ok(())
}


// Get One CrateApiEntity
// Using reqwest::Client
//
async fn get_one_crate_api_entity(client_req: &ReqwestClient, headers_api: &HeaderMap, crate_name: &str) -> Result<Option<Crate>, Box<dyn std::error::Error>> {

    let api_url = format!("{}{}", *CRATE_API, crate_name);
    info!("{}{}", "api_url: ", api_url);
    let response = client_req
        .get(&api_url).headers(headers_api.clone())
        .send()
        .await?
        .json::<CrateApiEntity>()
        .await?;
    if response.crates.len() == 0 {
    Ok(None)
    }
    else{
        Ok(Some(response.crates[0].clone()))
    }
}
