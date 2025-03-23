extern crate serde;

mod func;
mod load;

use std::env;
use std::path::Path;
use func::{add_todo, list_todos, mark_done, delete_todo, show_info, clean_old_tasks};
use load::load_or_create_config;
use clap::{Parser, Subcommand};

/* DATA */
#[derive(Parser)]
pub struct Config {
    pub data_file: String,
    pub archive_save_date: u32,
}

struct ToDoData {
    id: String,
    content: String,
    done: bool,
    date: String,
}

fn show_help() {
    println!("\n");
    println!("\t##    ##          #######        #####         ");
    println!("\t##    ##             #           #   ##        ");
    println!("\t###  # # #   #       #     ###   #    #   ###  ");
    println!("\t# #  # #  #  #       #    #  ##  #    ## #  ## ");
    println!("\t# #  # #  # #        #    #   #  #    ## #   # ");
    println!("\t#  ##  #   ##        #    #   #  #    #  #   # ");
    println!("\t#  ##  #   ##        #    #  ##  #   ##  #  ## ");
    println!("\t#      #   #         #     ###   #####    ###  ");
    println!("\t          ##                                   ");
    println!("\t          #                                    ");
    println!("==============================================================");
    println!("'add XXXX'  : Add a new task.");
    println!("'done XXXX' : Mark a task as done.");
    println!("'del XXXX'  : Delete a task.");
    println!("'info'      : Show info.");
    println!("'help'      : Show help.");
}

fn main() {
    /* DATA_INPORT */
    let config = load_or_create_config();

    /* MAIN */
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        list_todos(&config.data_file);
        return;
    }
    let command = &args[1];
    let data_file = Path::new(&config.data_file);

    clean_old_tasks(data_file, config.archive_save_date);

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add [content]");
                return;
            }
            let content = &args[2];
            add_todo(data_file, content);
        }
        "list" => list_todos(data_file),
        "done" => {
            if args.len() < 3 {
                println!("Usage: todo done [id]");
                return;
            }
            let id = &args[2];
            mark_done(data_file, id);
        }
        "del" => {
            if args.len() < 3 {
                println!("Usage: todo del [id]");
                return;
            }
            let id = &args[2];
            delete_todo(data_file, id);
        }
        "info" => show_info(data_file),
        "help" => show_help(),
        _ => println!("Unknown command: {}", command),
    }
}