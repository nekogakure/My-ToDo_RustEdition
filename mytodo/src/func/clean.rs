use crate::func::load_todo;
use crate::func::save_todo;
use chrono::Local;
use std::path::Path;
pub fn clean_old_tasks<P: AsRef<Path>>(data_file: P, archive_save_date: u32) {
    let mut todos = load_todo::load_todos(&data_file);
    let now = Local::now();
    todos.retain(|todo| {
        if todo.done {
            // Expected date format: "%Y-%m-%d %H:%M:%S.%f %:z" (e.g., "2025-04-18 20:56:04.010366555 +09:00")
            match chrono::DateTime::parse_from_str(&todo.date, "%Y-%m-%d %H:%M:%S%.f %:z") {
                Ok(task_date) => {
                    let duration = now.signed_duration_since(task_date);
                    duration.num_days() < archive_save_date as i64
                }
                Err(err) => {
                    // Log the error and keep the task (don't delete it if date parsing fails)
                    eprintln!(
                        "Warning: Failed to parse date '{}': {}. Keeping the task.",
                        todo.date, err
                    );
                    true
                }
            }
        } else {
            true
        }
    });
    save_todo::save_todos(&data_file, &todos);
}
