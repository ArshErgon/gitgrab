use std::collections::HashMap;

use colored::{self, Colorize};

fn print_logo(data_map: HashMap<String, &str>) {
    let msg = format!(
        r"             
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀                           
                                                        github@{username_top}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀      ⣀⣠⣤⣤⣤⣤⣤⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀{name}: {username}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀{comp}: {company}
⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀{loc}: {location}
⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⡇⠀⠀⠙⠻⠿⠛⠛⠛⠛⠛⠻⠿⠟⠁⠀⠀⢻⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀{bl}: {blog}
⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀{st}: {star}
⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀{fk}: {fork}
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⡇⠀⠀⠀{lg}: {top_lang}
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡇⠀⠀⠀{rep}: {repos}
⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀{foll}: {followers}
⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀{follw}: {following}
⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⡻⠿⣿⣿⣶⣦⣤⣀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣾⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀{bi}: {bio}
⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣆⠛⢿⣿⣿⣿⠋⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀{tw}: {twitter}
⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣿⣤⣄⣉⣉⣀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀{em}: {email}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀{cre}: {created}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⠿⠿⠀⠀⠀⠀⠀⠀⠀⠸⠿⠿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀{up}: {updated}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ",
        username_top = data_map.get("username").unwrap().bright_cyan().bold(),
        name = "Name".red().bold(),
        username = data_map.get("name").unwrap(),
        comp = "Company".red().bold(),
        company = data_map.get("company").unwrap(),
        loc = "Location".red().bold(),
        location = data_map.get("location").unwrap(),
        bl = "Blog".red().bold(),
        blog = data_map.get("blog").unwrap(),
        st = "Star".red().bold(),
        star = data_map.get("star").unwrap(),
        fk = "Fork".red().bold(),
        fork = data_map.get("fork").unwrap(),
        lg = "Top lang".red().bold(),
        top_lang = data_map.get("top_lang").unwrap(),
        rep = "Repos".red().bold(),
        repos = data_map.get("repos").unwrap(),
        foll = "Followers".red().bold(),
        followers = data_map.get("followers").unwrap(),
        follw = "Following".red().bold(),
        following = data_map.get("following").unwrap(),
        bi = "Bio".red().bold(),
        bio = data_map.get("bio").unwrap(),
        tw = "Twitter".red().bold(),
        twitter = data_map.get("twitter").unwrap(),
        em = "Email".red().bold(),
        email = data_map.get("email").unwrap(),
        cre = "Created".red().bold(),
        created = data_map.get("created").unwrap(),
        up = "Updated".red().bold(),
        updated = data_map.get("updated").unwrap(),
    );
    println!("{msg}");
}

pub fn print_formatter(
    header_git_data: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        i32,
        i32,
        i32,
        i32,
        String,
        String,
    )>,
    data_map: HashMap<String, u32>,
) {

    let mut git_map: HashMap<String, &str> = HashMap::new();
    git_map.insert("username".to_string(), header_git_data[0].0.as_str());
    git_map.insert("name".to_string(), header_git_data[0].1.as_str());
    git_map.insert("company".to_string(), header_git_data[0].2.as_str());
    git_map.insert("blog".to_string(), header_git_data[0].3.as_str());
    git_map.insert("location".to_string(), header_git_data[0].4.as_str());
    git_map.insert("bio".to_string(), header_git_data[0].5.as_str());
    git_map.insert("twitter".to_string(), header_git_data[0].6.as_str());
    git_map.insert("email".to_string(), header_git_data[0].7.as_str());
    let repo = add_k(header_git_data[0].8);

    let gists_two = add_k(header_git_data[0].9);
    let followers = add_k(header_git_data[0].10);
    let following = add_k(header_git_data[0].11);
    git_map.insert("repos".to_string(), repo.as_str());
    git_map.insert("gists".to_string(), gists_two.as_str());
    git_map.insert("followers".to_string(), followers.as_str());
    git_map.insert("following".to_string(), following.as_str());
    git_map.insert("created".to_string(), header_git_data[0].12.as_str());
    git_map.insert("updated".to_string(), header_git_data[0].13.as_str());
    let stars = add_k(data_map["Star"].try_into().unwrap());
    let forks = add_k(data_map["Fork"].try_into().unwrap());
    git_map.insert("star".to_string(), stars.as_str());
    git_map.insert("fork".to_string(), forks.as_str());

    let max_key = find_max_key(data_map);

    git_map.insert("top_lang".to_string(), max_key.as_str());

    print_logo(git_map);
}

fn find_max_key(data_map: HashMap<String, u32>) -> String {
    let mut max_val: u32 = 0;
    for (key, val) in data_map.clone() {
        if (!(key == "Star".to_string() || key == "Fork".to_string())) && val > max_val {
            max_val = val;
        }
    }

    let mut max_key = String::new();
    for (key, val) in data_map {
        if (!(key == "Star".to_string() || key == "Fork".to_string())) && val == max_val {
            max_key = key;
            break;
        }
    }

    max_key
}

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
