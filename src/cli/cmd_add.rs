use crate::{app::App, model::todo::Todo};
use clap::Parser;
use log::info;

#[derive(Parser)]
pub struct Args {
    /// The name of the todo to add.
    name: String,

    /// Whether the task is completed or not.
    #[arg(
        value_name = "completed",
        short = 'c',
        long = "completed",
        default_value = "false"
    )]
    is_completed: Option<bool>,
}

pub fn run(app: &mut App, args: Args) {
    let Args { name, is_completed } = args;

    let todo: Todo = Todo::new(name, is_completed);

    info!("Adding todo '{}' to database.", todo.name);

    let added_todo = app.add_todo(todo).unwrap();

    info!(
        "Added todo '{}' with id '{}' to database.",
        added_todo.name, added_todo.id
    );
}
