use crate::models::resident::Resident;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    resident_file_path: String,
    complaints_file_path: String,
    announcements_file_path: String,
}

impl Storage {
    pub fn new(resident_file_path: &str,complaints_file_path: &str,announcements_file_path: &str) -> Self {
        Self {
            resident_file_path: resident_file_path.to_string(),
            announcements_file_path: announcements_file_path.to_string(),
            complaints_file_path: complaints_file_path.to_string(),
        }
    }

    pub fn load_residents(&self) -> Vec<Resident> {
        if !Path::new(&self.resident_file_path).exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(&self.resident_file_path).unwrap_or("[]".to_string());

        serde_json::from_str(&data).unwrap_or(Vec::new())
    }

    pub fn save_residents(&self, resdents: &Vec<Resident>) {
        let data = serde_json::to_string_pretty(resdents).unwrap();
        fs::write(&self.resident_file_path, data).expect("Unable to write in file");
    }

    pub fn get_next_id(&self, residents: &Vec<Resident>) -> i64 {
        residents.iter().map(|r| r.get_id()).max().unwrap_or(0) + 1
    }
}
