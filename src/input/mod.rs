use std::{env, path::Path};

use clap::{Arg, ArgAction, Command};

mod menu_cli;

// this files do takes the argument from the cli
// it use clap

pub fn cli_input() {
    let matches = Command::new("gitfetch")
        .version("2.0")
        .author("A CLI application for github users, which shows the information of a particular user in a `neofetch` style\nProudly build with the help of Rust.")
        .about("Neofetch but build for GitHub")
        .arg(
            Arg::new("t")
                .long("t")
                .help("Show information for a temporary user: gitfetch -t <USER>"),
        )
        .arg(
            Arg::new("o")
                .long("o")
                .action(ArgAction::SetTrue)
                .help("Option to create the user or insert the github API key"),
        )
        .arg(
            Arg::new("author")
            .long("a")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    // for menu bar
    match matches.get_flag("o") {
        true => menu_cli::menu_view(),
        false => todo!(),
    };

    // a temporary user
    let arg_temp = match matches.get_one::<String>("t") {
        None => "None",
        Some(val) => val,
    };

    // to show the name of the user.
    if matches.get_flag("author") {
        about();
    };

    // start the temporary user function
    if arg_temp != "None" {}

    // menu_cli::menu_view();
}

pub fn show_user_info() -> (String, String) {
    let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    let apifile_path = Path::new(&home_dir).join("gitfetch_api.txt");
    let username_file_path = Path::new(&home_dir).join("gitfetch_user.txt");
    let secret_key = match std::fs::read_to_string(apifile_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{:?}", e);
            "Stop".to_string()
        }
    };

    let username = match std::fs::read_to_string(username_file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{:?}", e);
            "File not found".to_string()
        }
    };
    (username, secret_key)
}

// add some information about the creator
pub fn about() {
    let gitfetch_logo = format!(
        r"

         ██████╗ ██╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗
        ██╔════╝ ██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
        ██║  ███╗██║   ██║   █████╗  █████╗     ██║   ██║     ███████║
        ██║   ██║██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
        ╚██████╔╝██║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║
         ╚═════╝ ╚═╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝v.0.2.0

    A CLI application for github users, which shows the information of a particular user in a `neofetch` style
    Proudly build with the help of Rust.
    "
    );
    println!("{}", gitfetch_logo);
}
