// use clap::{arg, Command};
mod get_full_view;
mod github_logo_ascii;
mod input;
mod profile_header;

// TODO:
// use senseful and short names.
// document everything you are doing.
// use clap for making it a real CLI tool.
// make a startup page which tells the user about creating the token file.

// let see where this project will go?

fn main() {
    // let user = "ArshErgon";
    let user = input::propmt();
    let secret_key = "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".to_string();
    // printing the logo

    let header_git_data = profile_header::start_info(&user, secret_key.clone());
    let full_data = get_full_view::starrt(&user, secret_key.clone());
    github_logo_ascii::print_formatter(header_git_data, full_data);
    // language_bar::start_gen_lang_bar(full_data);
}
