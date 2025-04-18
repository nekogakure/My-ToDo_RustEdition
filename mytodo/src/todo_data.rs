use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoData {
    pub id: String,
    pub content: String,
    pub done: bool,
    pub date: String,
}
