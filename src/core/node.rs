use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub position: (f32, f32),
}

impl Node {
    pub fn new(id: u32, title: String, content: String, position: (f32, f32)) -> Self {
        Node {
            id,
            title,
            content,
            position,
        }
    }

    pub fn move_node(&mut self, new_position: (f32, f32)) {
        self.position = new_position;
    }
}
