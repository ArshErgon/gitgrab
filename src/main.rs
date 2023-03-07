#![allow(dead_code)]
#![allow(unused)]

use clap::Arg;
use clap::{arg, command, value_parser, ArgAction, Command};
mod get_detailed_view;
mod github_logo_ascii;
mod input;
mod profile_header;

mod graph {
    pub mod graph_maker;
}

use std::env;
use std::fs::{self};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    input::cli_input();
    start_the_project();
    // let matches = Command::new("MyApp")
    // .version("1.0")
    // .author("Kevin K. <kbknapp@gmail.com>")
    // .about("Does awesome things")
    // .arg(arg!(--t <i32>).required(true))
    // .arg(arg!(--one <VALUE>).required(true))
    // .arg(
    //     Arg::with_name("temp")
    //         .short("t")
    //         .long("temporally")
    //         .value_name("temp")
    //         .help("Show an user info temporally, ex: gitfetch -t USERNAME")
    //         .takes_value(true),
    // )
    // .arg(
    //     Arg::with_name("author")
    //         .short("a")
    //         .long("author")
    //         .value_name("author")
    //         .help("Show the information about the creator of gitfetch ex: gitfetch -a")
    //         .takes_value(false),
    // )
    //     .get_matches();
    // println!("{:?}", matches);

    // let author =
    //     "Gitfetch made by ArshErgon, an Open Source Developer who loves to help the community,
    // LinkedIn: https://www.linkedin.com/in/arsh-ergon"
    //         .to_string();

    // let arg_user = matches.value_of("username").unwrap_or("None");
    // let arg_temp = matches.value_of("temp").unwrap_or("None");

    // // a very rookie approach but I can't think something else, rightnow.
    // // a match would work I think

    // if matches.is_present("author") {
    //     println!("{author}");
    // } else if arg_temp != "None" {
    //     // for a temporary user.
    //     start_the_project(arg_temp);
    // } else if arg_user != "None" || matches.is_present("username") {
    //     // to configure a user to use without adding -u username next time.
    //     let home_dirs = env::var_os("HOME").unwrap();
    //     let file_path = home_dirs.into_string().unwrap() + "/gitFetchUser.txt";
    //     let mut file = match fs::OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .open(file_path)
    //     {
    //         Ok(file) => file,
    //         Err(e) => {
    //             println!("Error opening file: {:?}", e);
    //             return;
    //         }
    //     };
    //     file.write_all(arg_user.as_bytes()).unwrap();
    //     start_the_project(arg_user);
    // } else {
    //     // if the command is empty gitfetch.
    //     let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    //     let file_path = Path::new(&home_dir).join("gitFetchUser.txt");
    //     let error_msg = format!(
    //         "{oops} got an error. \
    //         This error happend because
    //         1. gitFetchUser.txt could be found.
    //         2. Or the Home Directory can not be located. \n
    //         gitfetch -u {username} or $ gitfetch -t {username}",
    //         oops = "Oops",
    //         username = "USERNAME",
    //     );
    //     let file = match fs::read_to_string(file_path) {
    //         Ok(contents) => contents,
    //         Err(e) => {
    //             println!("{error_msg}");
    //             return;
    //         }
    //     };
    //     start_the_project(file.as_str());
    // }
}

fn start_the_project() {
    // let home_dir = env::var_os("HOME").expect("Cannot get home directory!");
    // let file_path = Path::new(&home_dir).join("secretkey.txt");
    // let secret_key = match fs::read_to_string(file_path) {
    //     Ok(contents) => contents,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         return;
    //     }
    // };
    let (arg, secret_key) = input::show_user_info();
    let header_git_data = profile_header::start_header_info(arg.as_str(), secret_key.clone());
    let counter_data =
        get_detailed_view::main_view_start(arg.to_string(), secret_key.clone(), Some(false));
    let filter_data = counter_data.unwrap();
    github_logo_ascii::print_formatter(header_git_data, filter_data);
    // checker for graph contribution as it has an key error which needs to continously to be updated.
    let graph = get_detailed_view::show_contribution_graph(arg.to_string(), secret_key);
    match graph.unwrap_err() {
        error => println!("You should change you API key, it got expires for graph contribution\nits an issue: https://github.com/ArshErgon/gitfetch/issues/17"),
    }
}
