use dialoguer::{theme::ColorfulTheme, Select};

// todo:
// inside the create a user, check wheather the user has a github file or no?
// if no: suggest him to create a new (by asking him)
// if yes, print a message thats your name is save, by gitfetch you can get information.
// and how can he update that username

// in the enter/update github api, check the wheather it is exists
// if it exists show the user its current key
// and change the current key
pub fn menu_view() -> std::io::Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("1. Create a User")
        .item("2. Enter/Update the Github API key")
        .interact()?;

    match selection {
        0 => create_user_file(),
        1 => create_api_file(),
        _ => println!("Not an option"),
    };
    Ok(())
}

fn create_user_file() {
    let mut user_name = String::new();
    println!("Enter Username\n");
    let user_input = std::io::stdin();
    user_input.read_line(&mut user_name);
    let home_dir = std::env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = home_dir.clone().into_string().unwrap() + "/gitfetch_user.txt";
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // truncate the file to 0 bytes
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return;
        }
    };
    // add a checker to check wheather the user have api key file or not
    std::io::Write::write_all(&mut file, user_name.as_bytes()).unwrap();
    let success_msg = format!(
        "User file is sucessfuly created at {} with a name gitfetch_user\n 
    You can now run `gitfetch` to see your github information",
        home_dir.into_string().unwrap()
    );

    println!("{}", success_msg);
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
    let file_path = home_dir.into_string().unwrap() + "/gitfetch_api.txt";
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // truncate the file to 0 bytes
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return;
        }
    };
    std::io::Write::write_all(&mut file, api_key.as_bytes()).unwrap();
    match flag {
        true => println!("Key successfuly updated"),
        false => println!("Key file generted"),
    }
}

fn get_secret_key() -> (String, bool) {
    let home_dir = std::env::var_os("HOME").expect("Cannot get home directory!");
    let file_path = std::path::Path::new(&home_dir).join("secretkey.txt");
    let secret_key = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{:?}", e);
            "File not found".to_string()
        }
    };
    (secret_key, true)
}
