use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::collections::HashMap;
extern crate colorful;
use colorful::{Color, Colorful, HSL};
extern crate cfonts;
use cfonts::{say, Colors, Fonts, Options};

use crossterm::{
    execute,
    terminal::{self, SetSize},
};

// graph maker
use crate::graph::graph_maker;

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
}

#[tokio::main]
pub async fn get_repos_info(
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

fn gather_repo_info(user: &str, secret_key: String) -> HashMap<String, u32> {
    get_repos_info(user, secret_key).unwrap()
}

fn profile_header(user: String) {
    ascii_text(user);
    // will show the profile header information
    // information like name, contribution, total commit, total issues, close and opened,
}

fn rainbow() {
    let line =
        "\t\t\t███████████████████████████████████████████████████████████████████████████\n";
    line.rainbow();
}

fn progress_bar(data_map: HashMap<String, u32>) {
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

    let s = "█";
    ascii_text(String::from("Top Language"));

    let c = languages.iter().max_by_key(|x| x.len()).unwrap();

    for (i, value) in values.iter().enumerate() {
        let h = (*value as f32 * 15.0 % 360.0) / 360.0;
        let length = (value - 10.0) as usize;
        let length = if length >= 100 { length / 2 } else { length };
        println!(
            " {:<width$} | {} {}%\n",
            languages.get(i).unwrap(),
            s.repeat(length).gradient(HSL::new(h, 1.0, 0.5)),
            value,
            width = c.len()
        );
    }
}

fn clean_terminal() {
    // clean the full terminal of the terminal
    std::process::Command::new("clear").status().unwrap();
}

pub fn ascii_text(txt: String) {
    say(Options {
        text: txt,
        font: Fonts::FontTiny,
        colors: vec![Colors::YellowBright],
        align: cfonts::Align::Center,
        ..Options::default()
    });
}

fn set_new_terminal_size() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current size of the terminal window
    // let (width, height) = terminal::size()?;

    // Set the new size of the terminal window
    // the normal size of the window is default, using this because
    // bar size is increasing and doing a text wrapping
    // decreasing the length of the bar is decreasing all the other bars also.
    // at now setting a new terminal height is a solution
    let new_width = 124;
    let new_height = 50;
    let size = SetSize(new_width, new_height);
    execute!(std::io::stdout(), size)?;
    println!("Changing the size of the terminal\nWidth:{new_width}\t\tHeight:{new_height}\n\n");
    Ok(())
}

// check why does the contribution graph is not showing when using other keys.
pub fn show_contribution_graph(user_name: String, secret_key: String) {
    let secret_key = secret_key.trim();
    let key: &str = "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";
    graph_maker::generate_graph(user_name, key);
}

pub fn main_view_start(username: String, secret_key: String) -> HashMap<String, u32> {
    // clean the terminal
    clean_terminal();
    // change the size so that it can show bars and all that.
    set_new_terminal_size();
    clean_terminal();
    // An animated rainbow bar, attraction
    rainbow();
    // taking the stars, fork counts.
    let repo_data = get_repos_info(username.as_str(), secret_key.clone()).unwrap();
    // profile header bar, showing information about the user
    profile_header(username.clone());
    // starting the progress bar.
    progress_bar(repo_data.clone());
    // starting of the contribution graph
    ascii_text("Contribution Graph".to_string());
    show_contribution_graph(username, secret_key);
    repo_data
}