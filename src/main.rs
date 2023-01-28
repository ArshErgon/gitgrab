#![allow(dead_code)]
#![allow(unused)]

use clap::{App, Arg};
mod get_full_view;
mod github_logo_ascii;
mod profile_header;

// TODO:
// use senseful and short names.
// document everything you are doing.
// use clap for making it a real CLI tool.
// make a startup page which tells the user about creating the token file.

// let see where this project will go?

fn main() {
    let matches = App::new("Gitfetch")
                    .version("0.1.0")
                    .about("Just like `Neofetch` but for GitHub!")
                    .author("https://github.com/ArshErgon/gitfetch/")
                    .arg(Arg::with_name("user")
                            .short("u")
                            .long("user")
                            .value_name("USERNAME")
                            .help("ex: gitfetch USERNAME")
                            .conflicts_with("gif")
                            .takes_value(true))
                    .arg(Arg::with_name("temp")
                            .short("t")
                            .long("temporally")
                            .value_name("temp")
                            .help("Show an user info temporally")
                            .takes_value(true))
                    .get_matches();
    println!("{:?}", matches.value_of("temp").unwrap_or("None"));
    println!("{:?}", matches.value_of("user").unwrap_or("None"));


    let user = matches.value_of("user").unwrap_or("None");
    let user = "ArshErgon";
    let secret_key = "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".to_string();
    // printing the logo

    let header_git_data = profile_header::start_info(&user, secret_key.clone());
    let full_data = get_full_view::starrt(&user, secret_key.clone());
    github_logo_ascii::print_formatter(header_git_data, full_data);
    // language_bar::start_gen_lang_bar(full_data);
}
