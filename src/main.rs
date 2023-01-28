#![allow(dead_code)]
#![allow(unused)]

use clap::{App, Arg};
mod get_full_view;
mod github_logo_ascii;
mod profile_header;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

fn main() {
    let matches = App::new("Gitfetch")
        .version("0.1.0")
        .about("Just like `Neofetch` but for GitHub!")
        .author("https://github.com/ArshErgon/gitfetch/")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("user")
                .value_name("username")
                .help("saves the username, so that you don't have to put your username over again.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("temp")
                .short("t")
                .long("temporally")
                .value_name("temp")
                .help("Show an user info temporally, ex: ex: gitfetch -t USERNAME")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("author")
                .short("a")
                .long("author")
                .value_name("author")
                .help("Show the information about the creator of gitfetch ex: gitfetch -a")
                .takes_value(false),
        )
        .get_matches();

    let author =
        "Gitfetch made by ArshErgon, an Open Source Developer who loves to help the community, 
    LinkedIn: https://www.linkedin.com/in/arsh-ergon"
            .to_string();

    let arg_user = matches.value_of("username").unwrap_or("None");
    let arg_temp = matches.value_of("temp").unwrap_or("None");

    // a very rookie approach but I can't think something else, rightnow.
    // a match would work I think

    if matches.is_present("author") {
        println!("{author}");
    } else if arg_temp != "None" {
        // for a temporary user.
        start_the_project(arg_temp);
    } else if arg_user != "None" || matches.is_present("username") {
        // to configure a user to use without adding -u username next time.
        let home_dirs = env::var_os("HOME").unwrap();
        let file_path = home_dirs.into_string().unwrap() + "/gitfetchUser.txt";
        let mut file = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening file: {:?}", e);
                return;
            }
        };
        file.write_all(arg_user.as_bytes()).unwrap();
        start_the_project(arg_user);
    } else {
        // if the command is empty gitfetch.
        let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
        let file_path = Path::new(&home_dir).join("gitfetchUser.txt");
        let file = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("Oops! got an error can you please try: gitfetch -u YOUR_USERNAME. If the problem stays the same, write an issue here: https://github.com/ArshErgon/gitfetch/issues");
                return;
            }
        };
        start_the_project(file.as_str());
    }
}

fn start_the_project(arg: &str) {
    let secret_key = "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".to_string();
    let header_git_data = profile_header::start_info(arg, secret_key.clone());
    let full_data = get_full_view::starrt(arg, secret_key.clone());
    github_logo_ascii::print_formatter(header_git_data, full_data);
}
