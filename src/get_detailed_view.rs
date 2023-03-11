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

// Contribution Graph Maker
use crate::{graph::graph_maker, input};

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    stargazers_count: u32,
    forks_count: u32,
    language: Option<String>,
    open_issues_count: u32,
    watchers: u32,
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
        .map(|val| val.as_object().unwrap())
        .map(|val| serde_json::from_value(serde_json::Value::Object(val.clone())).unwrap())
        .collect();

    let data: Vec<(String, u32, u32, String, u32, u32)> = persons
        .iter()
        .map(|repo| {
            (
                repo.name.to_string(),
                repo.stargazers_count,
                repo.forks_count,
                repo.language.clone().unwrap_or_else(|| "NA".to_string()),
                repo.open_issues_count,
                repo.watchers,
            )
        })
        .collect();

    let length = data.len();

    // count the stars, forks, issues, watchers and languages
    let mut counter = HashMap::new();
    counter.insert("Star".to_string(), 0);
    counter.insert("Fork".to_string(), 0);
    counter.insert("Issue".to_string(), 0);
    counter.insert("Watcher".to_string(), 0);

    for i in 0..length {
        if data[i].3 != "NA".to_string() {
            let lang_count = counter.entry(data[i].3.clone()).or_insert(0);
            *lang_count += 1;
        }

        if data[i].1 > 0 {
            let star_count = counter.entry("Star".to_string()).or_insert(0);
            *star_count += data[i].1;
        }

        if data[i].2 > 0 {
            let fork_count = counter.entry("Fork".to_string()).or_insert(0);
            *fork_count += data[i].2;
        }

        if data[i].4 > 0 {
            let issue_count = counter.entry("Issue".to_string()).or_insert(0);
            *issue_count += data[i].2;
        }

        if data[i].5 > 0 {
            let watchers_count = counter.entry("Watcher".to_string()).or_insert(0);
            *watchers_count += data[i].5;
        }
    }

    // simple percentage for the top lang use.
    for (key, val) in counter.clone() {
        let percentage = ((val as f32 / 8 as f32) * 100.0) as u32;
        if !(key == "Star".to_string()
            || key == "Fork".to_string()
            || key == "Issue".to_string()
            || key == "Watcher")
        {
            counter.insert(key, percentage);
        }
    }

    Ok(counter)
}

fn gather_repo_info(user: &str, secret_key: String) -> HashMap<String, u32> {
    get_repos_info(user, secret_key).unwrap()
}

fn profile_header(user: String) {
    ascii_text(user);
}

fn rainbow() {
    let line =
        "\t\t\t███████████████████████████████████████████████████████████████████████████\n";
    line.rainbow();
}

// progress bar for languages bars.
fn progress_bar(data_map: HashMap<String, u32>) {
    let mut values = Vec::new();
    let mut languages = Vec::new();

    for (key, value) in data_map {
        if !(key == "Star" || key == "Fork" || key == "Issue" || key == "Watcher") {
            // progress_bar(key, value);
            if value > 100 {
                values.push(100.0);
            } else {
                values.push(value as f64);
            }
            languages.push(key);
        }
    }

    let bar = "█";
    ascii_text(String::from("Top Language"));

    let c = languages.iter().max_by_key(|x| x.len()).unwrap();

    for (i, value) in values.iter().enumerate() {
        let h = (*value as f32 * 15.0 % 360.0) / 360.0;
        let length = (value - 10.0) as usize;
        let length = if length >= 100 { length / 2 } else { length };
        println!(
            " {:<width$} | {} {}%\n",
            languages.get(i).unwrap(),
            bar.repeat(length).gradient(HSL::new(h, 1.0, 0.5)),
            value,
            width = c.len()
        );
    }
}

// clean the full terminal of the terminal
fn clean_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

// prints the ascii text to the terminal, colorful
fn ascii_text(txt: String) {
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
    Ok(())
}

// check why does the contribution graph is not showing when using other keys.
fn show_contribution_graph(user_name: String, secret_key: String) -> Result<(), anyhow::Error> {
    let secret_key = secret_key.trim();
    graph_maker::generate_graph(user_name, secret_key)
}

pub fn main_view_start() {
    // thinking to start all the function from right here.
    // the cli_input will be put inside here, with a variable
    // no need to pass username and secret_key as a parimeter
    // as the cli_input is already returing the user & key
    // we just need to start only this function and rest will work
    // make sure, some errors will happen, handle them
    // and document your code.
    let (username, secret_key) = input::cli_input();

    let header_git_data =
        crate::profile_header::start_header_info(username.clone().as_str(), secret_key.clone());
    let repo_data = get_repos_info(username.as_str(), secret_key.clone()).unwrap();

    // taking the stars, fork counts.
    // clean the terminal
    clean_terminal();
    // change the size so that it can show bars and all that.
    set_new_terminal_size();
    clean_terminal();
    // An animated rainbow bar, attraction
    rainbow();
    // profile header bar, showing information about the user
    profile_header(username.clone());
    crate::github_logo_ascii::print_formatter(header_git_data, repo_data.clone());
    // starting the progress bar.
    progress_bar(repo_data.clone());
    // starting of the contribution graph
    ascii_text("Contribution Graph".to_string());
    let graph = show_contribution_graph(username, secret_key);
    match graph.unwrap_err() {
        error => println!("You should change you API key, it got expires for graph contribution\nits an issue: https://github.com/ArshErgon/gitfetch/issues/17"),
    }
}
