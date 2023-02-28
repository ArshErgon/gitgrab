
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::{thread, time, time::Duration};

extern crate colorful;

use colorful::{Colorful, Color};
use colorful::HSL;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
}

#[tokio::main]
pub async fn start(
    user: &str,
    secret_key: String,
) -> Result<(HashMap<String, u32>), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_url = format!("https://api.github.com/users/{user}/repos");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, "{secret_key}".parse().unwrap());

    let response = client.get(&request_url).headers(headers).send().await?;

    let json_str = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&json_str)?;
    let persons: Vec<Repository> = json_data
        .as_array()
        .unwrap()
        .into_iter()
        .map(|v| v.as_object().unwrap())
        .map(|v| serde_json::from_value(serde_json::Value::Object(v.clone())).unwrap())
        .collect();

    let data: Vec<(String, u32, u32, String)> = persons
        .iter()
        .map(|x| {
            (
                x.name.to_string(),
                x.stargazers_count,
                x.forks_count,
                x.language.clone().unwrap_or_else(|| "NA".to_string()),
            )
        })
        .collect();

    let length = data.len();

    // count the stars, forks and Languages
    let mut star_lang_fork_count = HashMap::new();
    star_lang_fork_count.insert("Star".to_string(), 0);
    star_lang_fork_count.insert("Fork".to_string(), 0);
    for i in 0..length {
        if data[i].3 != "NA".to_string() {
            let count = star_lang_fork_count.entry(data[i].3.clone()).or_insert(0);
            *count += 1;
        }

        if data[i].1 > 0 {
            let star_count = star_lang_fork_count.entry("Star".to_string()).or_insert(0);
            *star_count += data[i].1;
        }

        if data[i].2 > 0 {
            let fork_count = star_lang_fork_count.entry("Fork".to_string()).or_insert(0);
            *fork_count += data[i].2;
        }
    }

    // simple percentage for the top lang use.
    // added a checker to not make percentage value for star count and fork count
    // will be using it later in the program as the program gets big
    for (key, val) in star_lang_fork_count.clone() {
        let percentage = ((val as f32 / 8 as f32) * 100.0) as u32;
        if !(key == "Star".to_string() || key == "Fork".to_string()) {
            star_lang_fork_count.insert(key, percentage);
        }
    }

    Ok(star_lang_fork_count)
}

pub fn start_full_view(user: &str, secret_key: String) -> HashMap<String, u32> {
    start(user, secret_key).unwrap()
}

pub fn printing_full_profile_view(data_map: HashMap<String, u32>) {
    let header_logo = format!(
        r"
    

     ██████╗ ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
    ██╔════╝ ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
    ██║  ███╗██║   ██║   █████╗  █████╗     ██║   ██║     ███████║
    ██║   ██║██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
    ╚██████╔╝██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
     ╚═════╝ ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝v.0.2.0  
 "
    );

    // name:
    // blog or bio should not be empty or should have a default values
    // top repos(by number of stars or by number of forks) even if 1 stars
    // add them.

    // iterate through key and value and pass them in the bottom function

    let mut values = Vec::new();
    let mut languages = Vec::new();

    for (key, value) in data_map {
        if !(key == "Star" || key == "Fork") {
            // progress_bar(key, value);
            if value > 100 {
                values.push(100.0);
            } else {
                values.push(value as f64);
            }
            languages.push(key);
        }
    }
    progress_bar(values, languages);
}

fn progress_bar(values: Vec<f64>, languages: Vec<String>) {
    let s = "█";
    let txt = "Top languages";
    txt.rainbow_with_speed(3);
    println!(" {}\n", txt.gradient(Color::Red));

    let c = languages.iter().max_by_key(|x| x.len()).unwrap();

    for (i, value) in values.iter().enumerate() {
        let h = (*value as f32 * 15.0 % 360.0) / 360.0;
        let length = (value - 30.0) as usize;
        println!(
            " {:<width$} | {} {}%\n",
            languages.get(i).unwrap(),
            s.repeat(length).gradient(HSL::new(h, 1.0, 0.5)),
            value,
            width = c.len()
        );
    }
}
