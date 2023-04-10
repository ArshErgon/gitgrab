use std::collections::HashMap;

use crate::{
    get_detailed_view::ascii_text, github_graphql::detailed_view::RepositoriesInformation,
};
use colorful::Colorful;
use term_table::{row::Row, table_cell::TableCell, Table};

struct GithubData {
    profile_data: HashMap<String, String>,
    language_data: HashMap<String, u32>,
    top_repo: HashMap<String, RepositoriesInformation>,
}

fn comparison() {
    let user_one = "ArshErgon".to_string();
    let user_two = "torvalds".to_string();

    ascii_text("Comparison".to_string());
    let userone_data = helper(user_one);
    let usertwo_data = helper(user_two.clone());
    let (userone_helper, userone_sum, username_one) = hardcode_function(userone_data);
    let (usertwo_helper, usertwo_sum, username_two) = hardcode_function(usertwo_data);
    creating_table(
        (userone_helper, userone_sum, username_one),
        (usertwo_helper, usertwo_sum, user_two),
    );
}

fn helper(user: String) -> GithubData {
    let (secret_key, _) = crate::input::menu_cli::get_secret_key();
    let github_data =
        crate::github_graphql::detailed_view::get_graphql_info(user, secret_key.trim());
    let github_filter = GithubData {
        profile_data: github_data.0,
        language_data: github_data.1,
        top_repo: github_data.2,
    };
    github_filter
}

fn hardcode_function(data: GithubData) -> (String, u32, String) {
    let personal_detail = data.profile_data;
    let language_detail = data.language_data;
    let name = &personal_detail["name"];
    let repos = &personal_detail["repo"];
    let issue = &personal_detail["issues"];
    let followers = &personal_detail["followers"];
    let following = &personal_detail["following"];
    let watcher = &personal_detail["watcher"];
    let star = &personal_detail["stars"];
    let pull_requests = &personal_detail["request"];
    let fork = &personal_detail["fork"];
    let username = &personal_detail["login"];
    let mut sum = (repos.parse::<u32>().unwrap()
        + issue.parse::<u32>().unwrap()
        + followers.parse::<u32>().unwrap()
        + followers.parse::<u32>().unwrap()
        + watcher.parse::<u32>().unwrap()
        + star.parse::<u32>().unwrap()
        + pull_requests.parse::<u32>().unwrap()
        + fork.parse::<u32>().unwrap());
    for (lang, count) in language_detail.clone() {
        sum += count
    }
    let repos = add_k(repos.parse::<u32>().unwrap());
    let issue = add_k(issue.parse::<u32>().unwrap());
    let followers = add_k(followers.parse::<u32>().unwrap());
    let following = add_k(following.parse::<u32>().unwrap());
    let watcher = add_k(watcher.parse::<u32>().unwrap());
    let star = add_k(star.parse::<u32>().unwrap());
    let pull_request = add_k(pull_requests.parse::<u32>().unwrap());
    let fork = add_k(fork.parse::<u32>().unwrap());
    let show_format = format!(
        "
Name        : {name} 
Repo count  : {repos}
Issue       : {issue}
Followers   : {followers}
Following   : {following}
Watcher     : {watcher}
Star        : {star}
PR          : {pull_requests}
Fork        : {fork}
Languages:
 {:#?}
",
        language_detail
    );
    (show_format.to_string(), sum, username.to_string())
}

fn creating_table(pair_one: (String, u32, String), pair_two: (String, u32, String)) {
    let (userone, userone_sum, userone_name) = (pair_one.0, pair_one.1, pair_one.2);
    let (usertwo, usertwo_sum, usertwo_name) = (pair_two.0, pair_two.1, pair_two.2);
    let mut table = Table::new();
    table.max_column_width = 100;
    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(
            userone_name.clone(),
            1,
            term_table::table_cell::Alignment::Center,
        ),
        TableCell::new_with_alignment(
            usertwo_name.clone(),
            1,
            term_table::table_cell::Alignment::Center,
        ),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(userone, 1, term_table::table_cell::Alignment::Left),
        TableCell::new_with_alignment(usertwo, 1, term_table::table_cell::Alignment::Left),
    ]));

    println!("{}", table.render());
    let (winner_name, loser_name) = if userone_sum > usertwo_sum {
        (userone_name, usertwo_name)
    } else {
        (usertwo_name, userone_name)
    };
    let winner_message = format!("
    Okay, so I've done calculation on the basis of contribution user `{winner}` is the winner.
    `{loser}` failed as the user could not make the impact as `{winner}` did.

    NOTE: I've calculated the winner on the basis of contribution, repos data like stars, forks and so on.
    ", winner=winner_name.color(colorful::Color::Aquamarine1a),
    loser=loser_name.color(colorful::Color::Aquamarine1a),
);
println!("{winner_message}");
}


fn add_k(num: u32) -> String {
    let ans = if num >= 1000 {
        let decimal_star = num as f32 / 1000.0;
        let num = format!("{:.1}k", decimal_star);
        num
    } else {
        num.to_string()
    };
    ans
}

pub fn start_comparison(pair: (String, String)) {
    let (userone, usertwo) = pair;
    comparison();
}
