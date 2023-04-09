use std::collections::HashMap;

use colorful::{Color, Colorful};
use term_table::Table;

fn print_logo(data_map: HashMap<String, String>) {
    let mut table = Table::new();
    table.max_column_width = 100;
    table.style = term_table::TableStyle::empty();
    let email = &data_map["email"];
    let repos = &data_map["repo"];
    let issue = &data_map["issue"];
    let followers = &data_map["followers"];
    let company = &data_map["company"];
    let watcher = &data_map["watcher"];
    let star = &data_map["stars"];
    let pull_requests = &data_map["request"];
    let fork = &data_map["fork"];
    let twitter = &data_map["twitter_username"];
    let name = &data_map["name"];
    let bio = &data_map["bio"];
    let blog = &data_map["website_url"];
    let following = &data_map["following"];
    let username = &data_map["login"];
    let location = &data_map["location"];
    let top_lang = &data_map["top_lang"];
    let update = &data_map["update"];
    let contribution = &data_map["contribution"];
    let msg = format!(
        r"
        {name} ({username}) has {repos} repos on GitHub, using {top_lang} as a top lang. 
        {followers} followers, {following} following, {star} stars, {fork} forks, and {watcher} watchers. 
        open {issue} issues and open {pull_request} pull requests and a doing contribution since {contribution}. 
        Works at {company} in {location}. 
        {bio} 
        Visit blog: {blog}.
        Contact: {email}. 
        Last updated: {updated}. 
        Follow on Twitter: {twitter}. ",
        name = name.clone().color(Color::Aquamarine1a),
        username = username.clone().color(Color::Aquamarine1a),
        email = email.clone().color(Color::Aquamarine1a),
        repos = repos.clone().color(Color::Aquamarine1a),
        issue = issue.clone().color(Color::Aquamarine1a),
        followers = followers.clone().color(Color::Aquamarine1a),
        company = company.clone().color(Color::Aquamarine1a),
        watcher = watcher.clone().color(Color::Aquamarine1a),
        star = star.clone().color(Color::Aquamarine1a),
        pull_request = pull_requests.clone().color(Color::Aquamarine1a),
        fork = fork.clone().color(Color::Aquamarine1a),
        twitter = twitter.clone().color(Color::Aquamarine1a),
        bio = bio.clone().color(Color::Aquamarine1a),
        blog = blog.clone().color(Color::Aquamarine1a),
        following = following.clone().color(Color::Aquamarine1a),
        location = location.clone().color(Color::Aquamarine1a),
        top_lang = top_lang.clone().color(Color::Aquamarine1a),
        updated = update.clone().color(Color::Aquamarine1a),
        contribution = contribution.clone().color(Color::Aquamarine1a),
    );
    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new_with_alignment(
            msg,
            2,
            term_table::table_cell::Alignment::Center,
        ),
    ]));

    println!("{}", table.render());
}

pub fn print_formatter(mut git_data: HashMap<String, String>, language_map: HashMap<String, u32>) {
    let repo = add_k(git_data[&"repo".to_string()].parse::<u32>().unwrap());
    git_data.entry("repo".to_string()).or_insert(repo);

    let followers = add_k(git_data[&"followers".to_string()].parse::<u32>().unwrap());
    let following = add_k(git_data[&"following".to_string()].parse::<u32>().unwrap());
    git_data.entry("followers".to_string()).or_insert(followers);
    git_data.entry("following".to_string()).or_insert(following);

    let stars = add_k(git_data[&"stars".to_string()].parse::<u32>().unwrap());
    let forks = add_k(git_data[&"fork".to_string()].parse::<u32>().unwrap());
    let issue = add_k(git_data[&"issues".to_string()].parse::<u32>().unwrap());
    let watcher = add_k(git_data[&"watcher".to_string()].parse::<u32>().unwrap());
    git_data.insert("stars".to_string(), stars);
    git_data.insert("fork".to_string(), forks);
    git_data.insert("issue".to_string(), issue);
    git_data.insert("watcher".to_string(), watcher);
    let max_key = language_map
        .iter()
        .max_by_key(|(_, &value)| value)
        .map(|(key, _)| key.clone())
        .unwrap_or_else(|| "NA".to_string());

    git_data.insert("top_lang".to_string(), max_key);

    print_logo(git_data);
}

// add a k to the number
// like 1000 will become 1k
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
