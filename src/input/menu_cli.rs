use dialoguer::{theme::ColorfulTheme, Select};

pub fn menu_view() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("1. Create a User")
        .item("2. Enter/Update the Github API key")
        .item("3. Exit")
        .interact()?;

    match selection {
        0 => create_user_file(),
        1 => create_api_file(),
        2 => {
            println!("Thanks for using fetchquest");
            std::process::exit(0)
        }
        _ => println!("Not an option"),
    };
    Ok(())
}

fn create_user_file() {
    let mut user_name = String::new();
    println!("Enter Username\n");
    let user_input = std::io::stdin();
    user_input.read_line(&mut user_name).unwrap();

    let home_dir = match std::env::var_os("HOME") {
        Some(path) => path,
        None => {
            println!("Cannot get home directory!");
            return;
        }
    };
    let file_path = match home_dir.to_str() {
        Some(path) => path.to_owned() + "/fetchquest_user.txt",
        None => {
            println!("Cannot convert home directory to string!");
            return;
        }
    };

    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return;
        }
    };

    std::io::Write::write_all(&mut file, user_name.as_bytes()).unwrap();

    let success_msg = format!(
        "\nUser file is successfully created at {} with a name fetchquest_user\n",
        home_dir.to_str().unwrap()
    );

    let flag = get_secret_key().1; // store just the boolean value.
    let mut flag_msg = "You can now run `fetchquest` to see your Github information.";

    if !flag {
        flag_msg = "API key is not available. Please create an API key at https://github.com/settings/tokens with 'repo' and 'user' scopes and store it in your home directory in a file named 'fetchquest_api.txt'.";
    }

    eprintln!("{0} {1}", success_msg, flag_msg);
    std::process::exit(0);
}

fn create_api_file() {
    let (key, flag) = get_secret_key();
    if flag {
        println!("Your current key: {key}");
    }
    let mut api_key = String::new();
    let api_input = std::io::stdin();
    api_input.read_line(&mut api_key);
    let home_dir = std::env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = home_dir.into_string().unwrap() + "/fetchquest_api.txt";
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // truncate the file to 0 bytes
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            std::process::exit(0)
        }
    };
    std::io::Write::write_all(&mut file, api_key.as_bytes()).unwrap();
    match flag {
        true => println!("\nKey successfuly updated"),
        false => println!("\nKey file generted"),
    }
    std::process::exit(0);
}

pub fn get_secret_key() -> (String, bool) {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Cannot get home directory!");
            std::process::exit(0);
        }
    };
    let file_path = home_dir.join("fetchquest_api.txt");
    let secret_key = match std::fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!(
                "\nAPI key not found at {}; see: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token and give it all permission (expect: deleting or creating)",
                file_path.display()
            );
            std::process::exit(0)
        }
    };
    (secret_key, true)
}
