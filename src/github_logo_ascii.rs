use std::collections::HashMap;

use colorful::{Color, Colorful};
use term_table::Table;

fn print_logo(data_map: HashMap<String, String>) {
    let mut table = Table::new();
    table.max_column_width = 40;
    table.style = term_table::TableStyle::rounded();

    let username = data_map.get("username").unwrap().as_str();
    let name = data_map.get("name").unwrap().as_str();
    let company = data_map.get("company").unwrap().as_str();
    let location = data_map.get("location").unwrap().as_str();
    let blog = data_map.get("blog").unwrap().as_str();
    let star = data_map.get("star").unwrap().as_str();
    let fork = data_map.get("fork").unwrap().as_str();
    let top_lang = data_map.get("top_lang").unwrap().as_str();
    let repos = data_map.get("repos").unwrap().as_str();
    let followers = data_map.get("followers").unwrap().as_str();
    let following = data_map.get("following").unwrap().as_str();
    let bio = data_map.get("bio").unwrap().as_str();
    let twitter = data_map.get("twitter").unwrap().as_str();
    let email = data_map.get("email").unwrap().as_str();
    let issue = data_map.get("watcher").unwrap().as_str();
    let watcher = data_map.get("watcher").unwrap().as_str();
    // there's a way of centering it, {^50}
    // but it was not working
    let logo = format!(
        "
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣴⣶⣾⣿⣿⣿⣿⣿⣿⣿⣷⣶⣦⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⠛⠛⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠛⠛⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠙⠛⠋⠉⠉⠉⠉⠉⠉⠉⠙⠛⠋⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣾⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣷⣄⡀⠉⢻⣿⣿⣿⣿⣶⣶⣦⠄⠀⠀⠀⠀⠀⠀⠀⠠⣴⣶⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣿⣿⣿⣷⡄⠀⠙⢿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣄⠀⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿⣷⣦⣤⣤⣤⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⢿⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⡿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        "
    );
    let msg = format!(
        r"
{0}`({10})` has {1} repositories with a total of {2} stars, {3} forks {9} watchers. 

Additionally, {0} has {4} open issues and {5} watchers. 

{0} is also following {6} accounts and has {7} followers, with a Top language {8}. 

Bio: {11}, 
blog: {12}, 
email: {13}, 
twitter: {14}",
        name.color(Color::Aquamarine1a),
        repos.color(Color::Aquamarine1a),
        star.color(Color::Aquamarine1a),
        fork.color(Color::Aquamarine1a),
        issue.color(Color::Aquamarine1a),
        watcher.color(Color::Aquamarine1a),
        following.color(Color::Aquamarine1a),
        followers.color(Color::Aquamarine1a),
        top_lang.color(Color::Aquamarine1a),
        watcher.color(Color::Aquamarine1a),
        username.color(Color::Aquamarine1a),
        bio.color(Color::Aquamarine1a),
        blog.color(Color::Aquamarine1a),
        email.color(Color::Aquamarine1a),
        twitter.color(Color::Aquamarine1a),
    );
    print!("{}\n", logo.color(Color::White));
    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new_with_alignment(
            msg,
            2,
            term_table::table_cell::Alignment::Left,
        ),
    ]));
    println!("{}", table.render());
}

// inserting the vector data which we get from header_info, to a map, as getting data would be readable and easy from it, then remembering the index of data
pub fn print_formatter(header_git_data: Vec<String>, data_map: HashMap<String, u32>) {
    let mut git_map: HashMap<String, String> = HashMap::new();
    git_map.insert("username".to_string(), header_git_data[0].clone());
    git_map.insert("name".to_string(), header_git_data[1].clone());
    git_map.insert("company".to_string(), header_git_data[2].clone());
    git_map.insert("blog".to_string(), header_git_data[3].clone());
    git_map.insert("location".to_string(), header_git_data[4].clone());
    git_map.insert("bio".to_string(), header_git_data[5].clone());
    git_map.insert("twitter".to_string(), header_git_data[6].clone());
    git_map.insert("email".to_string(), header_git_data[7].clone());
    let repo = add_k(header_git_data[8].parse::<i32>().unwrap());

    let gists_two = add_k(header_git_data[9].parse::<i32>().unwrap());
    let followers = add_k(header_git_data[10].parse::<i32>().unwrap());
    let following = add_k(header_git_data[11].parse::<i32>().unwrap());

    git_map.insert("repos".to_string(), repo);
    git_map.insert("gists".to_string(), gists_two);
    git_map.insert("followers".to_string(), followers);
    git_map.insert("following".to_string(), following);
    git_map.insert("created".to_string(), header_git_data[12].clone());
    git_map.insert("updated".to_string(), header_git_data[13].clone());
    let stars = add_k(data_map["Star"].try_into().unwrap());
    let forks = add_k(data_map["Fork"].try_into().unwrap());
    let issue = add_k(data_map["Issue"] as i32);
    let watcher = add_k(data_map["Watcher"] as i32);
    git_map.insert("star".to_string(), stars);
    git_map.insert("fork".to_string(), forks);
    git_map.insert("issue".to_string(), issue);
    git_map.insert("watcher".to_string(), watcher);

    let max_key = find_max_key(data_map);

    git_map.insert("top_lang".to_string(), max_key.to_string());

    print_logo(git_map);
}

// find the max_key for the top language
fn find_max_key(data_map: HashMap<String, u32>) -> String {
    let mut max_val: u32 = 0;
    let mut max_key = String::new();
    for (key, val) in data_map {
        if (!(key == "Star".to_string()
            || key == "Fork".to_string()
            || key == "Issue".to_string()
            || key == "Watcher".to_string()))
            && val > max_val
        {
            max_val = val;
            max_key = key;
        }
    }
    max_key
}

// add a k to the number
// like 1000 will become 1k
fn add_k(num: i32) -> String {
    let ans = if num >= 1000 {
        let decimal_star = num as f32 / 1000.0;
        let num = format!("{:.1}k", decimal_star);
        num
    } else {
        num.to_string()
    };
    ans
}
