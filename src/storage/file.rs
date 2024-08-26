use crate::core::mind_map::MindMap;
use serde_json::{from_str, to_string_pretty};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: String) -> Self {
        FileStorage { file_path }
    }

    pub fn save(&self, mind_map: &MindMap) {
        let json = to_string_pretty(mind_map).unwrap();
        let mut file = File::create(&self.file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load(&self) -> MindMap {
        let mut file = OpenOptions::new().read(true).open(&self.file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        from_str(&content).unwrap()
    }
}
