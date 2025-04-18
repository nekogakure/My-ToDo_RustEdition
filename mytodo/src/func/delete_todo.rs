use crate::func::load_todo;
use crate::func::save_todo;
use std::path::Path;
use uuid::Uuid;

pub fn delete_todo<P: AsRef<Path>>(data_file: P, id: &str) {
    let mut todos = load_todo::load_todos(&data_file);

    let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, id.as_bytes()).to_string();

    if let Some(o) = todos
        .iter()
        .position(|todo| dbg!(dbg!(&todo.id) == dbg!(&id)))
    {
        todos.remove(o);
    } else {
        eprintln!("todo not found");
        std::process::exit(1);
    }

    save_todo::save_todos(&data_file, &todos);
}
