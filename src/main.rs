#![allow(dead_code)]
#![allow(unused)]

mod get_detailed_view;
mod github_logo_ascii;
mod input;
mod profile_header;

mod graph {
    pub mod graph_maker;
}

// you can transfer the `start_the_project` to some other fuction which invokes every function
fn main() {
    let (username, secret_key) = input::cli_input();
    start_the_project(username, secret_key);
}

// fix the below overcomplicated code.
// as you can see secretkey is being used in 3 different function, make it good.
// filter out non-useful variables and functions
// :: dont ask for
fn start_the_project(arg: String, secret_key: String) {
    let header_git_data = profile_header::start_header_info(arg.as_str(), secret_key.clone());
    let counter_data = get_detailed_view::main_view_start(arg.to_string(), secret_key.clone());
    github_logo_ascii::print_formatter(header_git_data, counter_data);
}
