use std::collections::HashMap;
extern crate colorful;
use colorful::{Colorful, HSL};
extern crate cfonts;
use cfonts::{say, Colors, Fonts, Options};

use crossterm::{
    execute,
    terminal::{self, SetSize},
};

// Contribution Graph Maker
use crate::{graph::graph_maker, input};

use crate::github_graphql::detailed_view;

fn profile_header(user: String) {
    ascii_text(user);
}

fn rainbow() {
    let line = "\t\t███████████████████████████████████████████████████████████████████████████\n";
    line.rainbow();
}

// progress bar for languages bars.
fn progress_bar(data_map: HashMap<String, i32>) {
    let mut values = Vec::new();
    let mut languages = Vec::new();

    for (key, value) in data_map {
        // progress_bar(key, value);
        if value > 100 {
            values.push(100.0);
        } else {
            values.push(value as f64);
        }
        languages.push(key);
    }

    let bar = "█";
    ascii_text(String::from("Top Language"));

    let c = languages.iter().max_by_key(|x| x.len()).unwrap();

    for (i, value) in values.iter().enumerate() {
        let h = (*value as f32 * 15.0 % 360.0) / 360.0;
        let length = (value - 10.0) as usize;
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
    // Set the new size of the terminal window
    // the normal size of the window is default, using this because
    // bar size is increasing and doing a text wrapping
    // decreasing the length of the bar is decreasing all the other bars also.
    // at now setting a new terminal height is a solution
    let new_width = 110;
    let new_height = 30;
    let size = SetSize(new_width, new_height);
    execute!(std::io::stdout(), size)?;
    Ok(())
}

// check why does the contribution graph is not showing when using other keys.
fn show_contribution_graph(user_name: String, secret_key: String) -> Result<(), anyhow::Error> {
    let secret_key = secret_key.trim();
    graph_maker::generate_graph(user_name, secret_key)
}

// the main_view_start is the backbone of our tool.
// the two username and secret_key grab the github username, and the API key
// API key always saved in a .txt file inside the `home_dir`
// same goes for the permanent user, the only time the username file will not be read when the command is starts with -t

// the header_git_data: takes a vector of string which is fetching the basic information like username, repo counts etc from the file `start_header_info`

// the repo_data is holding the repo details, like total stars counts etc (graphql will help me alot here, need an improment)
pub fn main_view_start() {
    let (username, secret_key) = input::cli_input();
    let (profile_data, language_data) =
        detailed_view::get_graphql_info(username.clone(), secret_key.trim());

    // change the size so that it can show bars and all that.

    set_new_terminal_size();
    clean_terminal();

    // An animated rainbow bar, attraction

    rainbow();

    // profile header bar, showing information about the user
    // prints the github logo and the basic information

    profile_header(username.clone());
    crate::github_logo_ascii::print_formatter(profile_data, language_data.clone());

    // starting the progress bar.
    // for languages it will start a bar.

    progress_bar(language_data);

    // starting of the contribution graph
    // ascii_text converts text to ascii art for heading

    ascii_text("Contribution Graph".to_string());
    let graph = show_contribution_graph(username, secret_key);
    match graph {
        Ok(()) => print!(""),
        Err(error) => {
            eprintln!("You should change you API key, it got expires for graph contribution\nits an issue: https://github.com/ArshErgon/gitfetch/issues/17");
            std::process::exit(0)
        }
    }
}
