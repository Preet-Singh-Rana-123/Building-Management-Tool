use crate::models::resident::Resident;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn load_residents(&self) -> Vec<Resident> {
        if !Path::new(&self.file_path).exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(&self.file_path).unwrap_or("[]".to_string());

        serde_json::from_str(&data).unwrap_or(Vec::new())
    }

    pub fn save_residents(&self, resdents: &Vec<Resident>) {
        let data = serde_json::to_string_pretty(resdents).unwrap();
        fs::write(&self.file_path, data).expect("Unable to write in file");
    }

    pub fn get_next_id(&self, residents: &Vec<Resident>) {
        residents.iter().map(|r| r.get_id()).max().unwrap_or(0) + 1;
    }
}
