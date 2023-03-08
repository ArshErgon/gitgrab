use dialoguer::{theme::ColorfulTheme, Select};

enum UserOption {
    User,
    Api,
    About,
    Commands,
}

// todo:
// inside the create a user, check wheather the user has a github file or no?
// if no: suggest him to create a new (by asking him)
// if yes, print a message thats your name is save, by gitfetch you can get information.
// and how can he update that username

// in the enter/update github api, check the wheather it is exists
// if it exists show the user its current key
// and change the current key
pub fn menu_view() {
    let options = [
        "1. Create a User",
        "2. Enter/Update the Github API key",
        "3. About",
        "4. Show all commands",
    ];
    let selected_option = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option:")
        .items(&options)
        .interact()
        .unwrap();
    let args = match options[selected_option] {
        "1. Create a User" => UserOption::User,
        "2. Enter the Github API key" => UserOption::Api,
        "3. About" => UserOption::About,
        "4. Show all commands" => UserOption::Commands,
        &_ => todo!(),
    };
    // add a match case which uses the enum
    // with parameter of args,
    // give each of them their function
    match args {
        UserOption::User => println!("User"),
        UserOption::Api => println!("API"),
        UserOption::About => println!("About"),
        UserOption::Commands => println!("Commands"),
    };
}
