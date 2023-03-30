#![allow(dead_code)]
#![allow(unused)]

use get_detailed_view::main_view_start;

mod get_detailed_view;
mod github_logo_ascii;
mod input;
mod github_graphql {
    pub mod detailed_view;
}
mod graph {
    pub mod graph_maker;
}

// the main function starts here.
// 1. Inside the main_view_start
// .. I'm using starting the whole project every function in different files starts there.
// more information inside the function
fn main() {
    main_view_start();
}
