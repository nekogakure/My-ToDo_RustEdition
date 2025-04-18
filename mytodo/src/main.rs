mod func;
mod help;
mod todo_data;
use clap::{Parser, Subcommand};
use func::config;
use std::path::Path;

/* DATA */
#[derive(Parser)]
#[clap(name = "My-ToDo_CLI", disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    to_do_command: Option<ToDoCommand>,
}

#[derive(Subcommand)]
enum ToDoCommand {
    /// Add a new task
    Add { content: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { id: String },
    /// Delete a task
    Del { id: String },
    /// Show info about the program
    Info,
    /// Show help message
    Help,
}

fn main() {
    // DATA_INPORT
    let config = config::load_or_create_config();

    // MAIN
    let args = Cli::parse();

    let Some(args) = args.to_do_command else {
        func::list_todo::list_todos(&config.data_file);
        return;
    };

    let data_file_path: &Path = config.data_file.as_ref();

    func::clean::clean_old_tasks(data_file_path, config.archive_save_date);

    match args {
        ToDoCommand::Add { content } => {
            func::add_todo::add_todo(&config.data_file, &content);
            func::list_todo::list_todos(&config.data_file);
        }
        ToDoCommand::List => {
            func::list_todo::list_todos(&config.data_file);
        }
        ToDoCommand::Done { id } => {
            func::done_todo::mark_done(&config.data_file, &id);
            func::list_todo::list_todos(&config.data_file);
        }
        ToDoCommand::Del { id } => {
            func::delete_todo::delete_todo(&config.data_file, &id);
            func::list_todo::list_todos(&config.data_file);
        }
        ToDoCommand::Info => {
            func::info::show_info();
        }
        ToDoCommand::Help => {
            help::show_help();
        }
    }
}
