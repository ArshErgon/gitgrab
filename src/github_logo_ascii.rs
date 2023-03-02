use std::collections::HashMap;

use colorful::{Colorful, Color};

fn print_logo(data_map: HashMap<String, &str>) {    
    let msg = format!(
        r"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                                        github@{username_top}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀      ⣀⣠⣤⣤⣤⣤⣤⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀------------------------------
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀    {name}: {username}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀    {comp}: {company}
⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀    {loc}: {location}
⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⡇⠀⠀⠙⠻⠿⠛⠛⠛⠛⠛⠻⠿⠟⠁⠀⠀⢻⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀    {bl}: {blog}
⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀    {st}: {star}
⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀    {fk}: {fork}
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⡇⠀⠀⠀    {lg}: {top_lang}
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⡇⠀⠀⠀    {rep}: {repos}
⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀    {foll}: {followers}
⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀    {follw}: {following}
⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⡻⠿⣿⣿⣶⣦⣤⣀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣾⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀    {bi}: {bio}
⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣆⠛⢿⣿⣿⣿⠋⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀    {tw}: {twitter}
⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣿⣤⣄⣉⣉⣀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀    {em}: {email}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀    {cre}: {created}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⠿⠿⠀⠀⠀⠀⠀⠀⠀⠸⠿⠿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀    {up}: {updated}

⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ {red_bright}{green_bright}{yellow_bright}{blue_bright}{purple_bright}{white_bright}
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ {light_red}{light_green}{light_yellow}{light_blue}{light_purple}{light_white}
    ",
        username_top = data_map.get("username").unwrap().to_string().color(Color::Cyan2),
        name = "Name".to_string().color(Color::Red),
        username = data_map.get("name").unwrap(),
        comp = "Company".to_string().color(Color::Red),
        company = data_map.get("company").unwrap(),
        loc = "Location".to_string().color(Color::Red),
        location = data_map.get("location").unwrap(),
        bl = "Blog".to_string().color(Color::Red),
        blog = data_map.get("blog").unwrap(),
        st = "Star".to_string().color(Color::Red),
        star = data_map.get("star").unwrap(),
        fk = "Fork".to_string().color(Color::Red),
        fork = data_map.get("fork").unwrap(),
        lg = "Top lang".to_string().color(Color::Red),
        top_lang = data_map.get("top_lang").unwrap(),
        rep = "Repos".to_string().color(Color::Red),
        repos = data_map.get("repos").unwrap(),
        foll = "Followers".to_string().color(Color::Red),
        followers = data_map.get("followers").unwrap(),
        follw = "Following".to_string().color(Color::Red),
        following = data_map.get("following").unwrap(),
        bi = "Bio".to_string().color(Color::Red),
        bio = data_map.get("bio").unwrap(),
        tw = "Twitter".to_string().color(Color::Red),
        twitter = data_map.get("twitter").unwrap(),
        em = "Email".to_string().color(Color::Red),
        email = data_map.get("email").unwrap(),
        cre = "Created".to_string().color(Color::Red),
        created = data_map.get("created").unwrap(),
        up = "Updated".to_string().color(Color::Red),
        updated = data_map.get("updated").unwrap(),
        red_bright = "████".to_string().color(Color::Red3b),
        green_bright = "████".to_string().color(Color::Green),
        yellow_bright = "████".to_string().color(Color::Yellow),
        blue_bright = "████".to_string().color(Color::Blue),
        purple_bright = "████".to_string().color(Color::Purple1a),
        white_bright = "████".to_string().color(Color::White),
        light_red = "████".to_string().color(Color::LightRed),
        light_green = "████".to_string().color(Color::LightGreen),
        light_yellow = "████".to_string().color(Color::LightYellow),
        light_blue = "████".to_string().color(Color::LightBlue),
        light_purple = "████".to_string().color(Color::MediumPurple4),
        light_white = "████".to_string().color(Color::Wheat4),
    );
    println!("{msg}");
}

pub fn print_formatter(header_git_data: Vec<String>, data_map: HashMap<String, u32>) {
    let mut git_map: HashMap<String, &str> = HashMap::new();
    git_map.insert("username".to_string(), header_git_data[0].as_str());
    git_map.insert("name".to_string(), header_git_data[1].as_str());
    git_map.insert("company".to_string(), header_git_data[2].as_str());
    let blog_value = if header_git_data[3].len() > 30 {
        header_git_data[3][0..25].as_ref()
    } else {
        header_git_data[3].as_str()
    };
    git_map.insert("blog".to_string(), blog_value);
    git_map.insert("location".to_string(), header_git_data[4].as_str());
    let bio_value = if header_git_data[5].len() > 30 {
        header_git_data[5][0..25].as_ref()
    } else {
        header_git_data[5].as_str()
    };
    git_map.insert("bio".to_string(), bio_value);
    git_map.insert("twitter".to_string(), header_git_data[6].as_str());
    git_map.insert("email".to_string(), header_git_data[7].as_str());
    let repo = add_k(header_git_data[8].parse::<i32>().unwrap());

    let gists_two = add_k(header_git_data[9].parse::<i32>().unwrap());
    let followers = add_k(header_git_data[10].parse::<i32>().unwrap());
    let following = add_k(header_git_data[11].parse::<i32>().unwrap());
    git_map.insert("repos".to_string(), repo.as_str());
    git_map.insert("gists".to_string(), gists_two.as_str());
    git_map.insert("followers".to_string(), followers.as_str());
    git_map.insert("following".to_string(), following.as_str());
    git_map.insert("created".to_string(), header_git_data[12].as_str());
    git_map.insert("updated".to_string(), header_git_data[13].as_str());
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
    let mut max_key = String::new();
    for (key, val) in data_map {
        if (!(key == "Star".to_string() || key == "Fork".to_string())) && val > max_val {
            max_val = val;
            max_key = key;
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
