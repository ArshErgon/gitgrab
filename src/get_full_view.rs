use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::collections::HashMap;
extern crate colorful;
use colorful::Colorful;
use colorful::{Color, HSL};
extern crate cfonts;
use cfonts::{say, Colors, Fonts, Options};

use crossterm::{
    execute,
    terminal::{self, SetSize},
};

#[derive(Debug, Deserialize)]
struct ContributionData {
    date: String,
    count: i32,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
    languages_url: Option<HashMap<String, i32>>,
    description: Option<String>,
    open_issues: i32,
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

#[tokio::main]
async fn fetch_contributions_data() -> Vec<ContributionData> {
    // add parameters for username and secretKey
    let user = "ArshErgon";
    let secret_key = "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";
    let client = Client::new();
    let url = format!("https://api.github.com/users/{user}/events");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, "{secret_key}".parse().unwrap());

    let response = client.get(&url).headers(headers).send().await.unwrap();
    let body = response.text().await.unwrap();
    let events: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&body).unwrap();
    let mut contributions = vec![];
    for event in events {
        if let Some(type_) = event.get("type") {
            if type_.as_str().unwrap() == "PushEvent" {
                let date = event.get("created_at").unwrap().as_str().unwrap()[0..10].to_string();
                let count = event
                    .get("payload")
                    .unwrap()
                    .get("size")
                    .unwrap()
                    .as_i64()
                    .unwrap() as i32;
                contributions.push(ContributionData { date, count });
            }
        }
    }
    contributions
}

pub fn start_full_view(user: &str, secret_key: String) -> HashMap<String, u32> {
    start(user, secret_key).unwrap()
}

fn profile_header() {
    ascii_text(String::from("Profile header"));
    // will show the profile header information
    // information like name, contribution, total commit, total issues, close and opened,
}

pub fn printing_full_profile_view(data_map: HashMap<String, u32>) {
    set_new_terminal_size();
    clean_terminal();
    let line =
        "\t\t\t███████████████████████████████████████████████████████████████████████████\n";
    line.rainbow();
    profile_header();
    progress_bar(data_map);
    let contribution_count = show_contribution_graph();
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

fn show_contribution_graph() -> i32 {
    ascii_text("Contribution Graph".to_string());
    // Get the user's contributions data from GitHub API
    let contributions_data = fetch_contributions_data();

    // Define the characters to represent the contribution graph
    let ascii_char = "██";

    // Loop through the contributions data and draw the graph
    let mut contribution_total = 0;
    for day in contributions_data {
        let contribution_count = day.count;
        contribution_total += contribution_count;
        // Set the color based on the contribution count
        let colors = match contribution_count {
            0 => Color::White,
            1..=10 => Color::Yellow,
            11..=20 => Color::Aquamarine3,
            21..=30 => Color::Blue,
            31..=40 => Color::Pink1,
            41..=50 => Color::Green,
            51..=100 => Color::Blue,
            _ => Color::Red,
        };

        //    print!("{}", ascii_char.to_string().color(colors));
    }
    println!("{contribution_total}");
    contribution_total
}



// new Function for getting the counting the total number of issues (open and closed)
fn fetching_issues() {
    // add the perimeters for username and secretKey
    let user = "ArshErgon";
    let secret_key = "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";
    let client = Client::new();
    let url = format!("https://api.github.com/repos/:owner/:repo/issues?state=all");

}