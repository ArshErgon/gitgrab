use std::{env, path::Path};

use clap::{Arg, ArgAction, Command};

mod menu_cli;

pub fn cli_input() -> (String, String) {
    let mut flag = false;
    let matches = Command::new("gitfetch")
        .version("2.0")
        .author("A CLI application for github users, which shows the information of a particular user in a `neofetch` style\nProudly build with the help of Rust.")
        .about("Neofetch but build for GitHub")
        .arg(
            Arg::new("t")
                .short('t')
                .long("temp")
                .help("Show information for a temporary user: gitfetch -t <USER>"),
        )
        .arg(
            Arg::new("o")
                .short('o')
                .long("option")
                .action(ArgAction::SetTrue)
                .help("Option to create the user or insert the github API key"),
        )
        .arg(
            Arg::new("author")
            .short('a')
            .long("author")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    // for menu bar
    match matches.get_flag("o") {
        true => menu_cli::menu_view(),
        false => Ok(()),
    };

    // a temporary user
    let arg_temp = match matches.get_one::<String>("t") {
        None => "None",
        Some(val) => val,
    };

    // to show the name of the user.
    if matches.get_flag("author") {
        about();
        flag = true;
    };

    // start the temporary user function
    let (mut username, mut secret_key) = (String::new(), String::new());

    if arg_temp != "None" {
        let (key, _) = menu_cli::get_secret_key();
        (username, secret_key) = (arg_temp.to_string(), key);
        flag = true;
    }

    // for the feature gitfetch
    // so that a full information about the permanet user(the home_dir/gitfetch_user one) will be displayed on the screen.
    if !flag {
        (username, secret_key) = show_user_info(String::new(), false);
    }
    (username, secret_key)
}

fn show_user_info(arg: String, flag: bool) -> (String, String) {
    let home_dir = match env::var_os("HOME") {
        Some(path) => path,
        None => {
            eprintln!("Cannot get home directory!");
            std::process::exit(0);
        }
    };

    let apifile_path = Path::new(&home_dir).join("gitfetch_api.txt");
    let username_file_path = Path::new(&home_dir).join("gitfetch_user.txt");
    let mut username = match std::fs::read_to_string(&username_file_path) {
        Ok(contents) => contents.trim().to_string(),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(0)
        }
    };

    let secret_key_string = menu_cli::get_secret_key().0; // get the string only
    if !secret_key_string.is_empty() {
        // overwrite the contents of gitfetch_api.txt with the new API key
        match std::fs::write(&apifile_path, secret_key_string.clone()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error writing to file: {:?}", e);
                std::process::exit(0)
            }
        }
    }

    (username, secret_key_string)
}

// add some information about the creator
fn about() {
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
    std::process::exit(0)
}
