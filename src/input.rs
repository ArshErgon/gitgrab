// TODO
// create a function which will take input from the user
// remember when to ask for the key and when to not,
// if the user just type gitfetch it should checks wheather the API key is present in the system.(handle a case when the API key get expires)
// if the api key can't be found inside the system, show him whats missing and how to create a ACCESS TOKEN.
// if present show him the details of HIS saved username(handle a case when he change his username)

use std::io::stdin;

fn read_user_input() -> String {
    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("failed to read from standard input");
    username
}

pub fn propmt() -> String {
    let name: String = read_user_input();
    name
}
