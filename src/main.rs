use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db_path = "data/db.json".to_owned();
    let db = Rc::new(JiraDatabase::new(db_path));
    let mut nav = Navigator::new(db);
    
    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user sanitized_input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action


        let current = nav.get_current_page();
        if current.is_none() {
            println!("No current page");
            break;
        }
        let current = current.unwrap();
        if let Err(error) = current.draw_page() {
            println!("Error rendering page: {}\nPress any key to continue", error);
            wait_for_key_press();
        };
        let input = get_user_input();
        match current.handle_input(&input) {
            Ok(action) => {
                if action.is_none() { 
                    println!("Action not recognized. Press any key to continue");
                    wait_for_key_press();
                } else {
                    nav.handle_action(action.unwrap()).unwrap();
                }
            },
            Err(e) => {
                println!("Error with action {e}: Press any key to continue");
                wait_for_key_press();
            }
        };
    }
}