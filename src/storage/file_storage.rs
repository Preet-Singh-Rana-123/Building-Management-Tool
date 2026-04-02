use crate::models::announcements::Announcements;
use crate::models::complaint::Complaints;
use crate::models::resident::Resident;
use crate::traits::identifiable::Identifiable;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    resident_file_path: String,
    complaints_file_path: String,
    announcements_file_path: String,
}

impl Storage {
    pub fn new(
        resident_file_path: &str,
        complaints_file_path: &str,
        announcements_file_path: &str,
    ) -> Self {
        Self {
            resident_file_path: resident_file_path.to_string(),
            announcements_file_path: announcements_file_path.to_string(),
            complaints_file_path: complaints_file_path.to_string(),
        }
    }

    // Handle Complaint data

    pub fn load_complaints(&self) -> Vec<Complaints> {
        if !Path::new(&self.complaints_file_path).exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(&self.complaints_file_path).unwrap_or("[]".to_string());

        serde_json::from_str(&data).unwrap_or(Vec::new())
    }

    pub fn save_complaints(&self, complaints: &Vec<Complaints>) {
        let data = serde_json::to_string_pretty(complaints).unwrap();
        fs::write(&self.complaints_file_path, data).expect("Unable to write in file");
    }

    // Handle Announcement data

    pub fn load_announcements(&self) -> Vec<Announcements> {
        if !Path::new(&self.announcements_file_path).exists() {
            return Vec::new();
        }

        let data = fs::read_to_string(&self.announcements_file_path).unwrap_or("[]".to_string());

        serde_json::from_str(&data).unwrap_or(Vec::new())
    }

    pub fn save_announcements(&self, announcements: &Vec<Announcements>) {
        let data = serde_json::to_string_pretty(announcements).unwrap();
        fs::write(&self.announcements_file_path, data).expect("Unable to write in file");
    }

    // Handle Resident data

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

    pub fn get_next_id<T: Identifiable>(&self, items: &Vec<T>) -> i64 {
        items.iter().map(|r| r.get_id()).max().unwrap_or(0) + 1
    }
}
