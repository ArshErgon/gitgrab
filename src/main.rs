#![allow(dead_code)]
#![allow(unused)]

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
}

// fix the below overcomplicated code.
// as you can see secretkey is being used in 3 different function, make it good.
// filter out non-useful variables and functions
// :: dont ask for
fn start_the_project() {
    let (arg, secret_key) = input::show_user_info();
    let header_git_data = profile_header::start_header_info(arg.as_str(), secret_key.clone());
    let counter_data = get_detailed_view::main_view_start(arg.to_string(), secret_key.clone());
    let filter_data = counter_data;
    github_logo_ascii::print_formatter(header_git_data, filter_data);
}
