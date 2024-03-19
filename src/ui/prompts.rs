use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("Epic Name:");
    let epic_name = get_user_input();
    println!("Epic Description");
    let epic_description = get_user_input();
    println!("{}", epic_description.to_string());
    Epic::new(epic_name, epic_description)
}

fn create_story_prompt() -> Story {
    println!("Story Name:");
    let story_name = get_user_input();
    println!("Story Description");
    let story_description = get_user_input();
    println!("{story_description}");
    Story::new(story_name, story_description)
}

fn delete_epic_prompt() -> bool {
    println!("Are you sure you want to delete this epic? (y/n)");
    let input = get_user_input();
    match input.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

fn delete_story_prompt() -> bool {
    println!("Are you sure you want to delete this story? (y/n)");
    let input = get_user_input();
    match input.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let input = get_user_input();
    match input.parse::<i32>() {
        Ok(1) => Some(Status::Open),
        Ok(2) => Some(Status::InProgress),
        Ok(3) => Some(Status::Resolved),
        Ok(4) => Some(Status::Closed),
        _ => None,
    }
}
