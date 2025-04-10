mod func;
mod load;
mod help;

use std::path::Path;
use func::{add_todo, list_todos, mark_done, delete_todo, show_info, clean_old_tasks};
use load::load_or_create_config;
use clap::{Parser, Subcommand};

/* DATA */
#[derive(Parser)]
#[clap(name = "My-ToDo_CLI", disable_help_subcommand = true)]
pub struct Config {
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
    let config = load_or_create_config();

    // MAIN
    let args = Config::parse();

    let Some(args) = args.to_do_command else {
        list_todos(&config.data_file);
        return;
    };

    let data_file_path: &Path = config.data_file.as_ref();

    clean_old_tasks(data_file_path, config.archive_save_date);

    match args{
        ToDoCommand::Add { content } => {
            add_todo(&config.data_file, &content);
        }
        ToDoCommand::List => {
            list_todos(&config.data_file);
        }
        ToDoCommand::Done { id } => {
            mark_done(&config.data_file, &id);
        }
        ToDoCommand::Del { id } => {
            delete_todo(&config.data_file, &id);
        }
        ToDoCommand::Info => {
            show_info();
        }
        ToDoCommand::Help => {
            help::show_help();
        }
    }    
        

}