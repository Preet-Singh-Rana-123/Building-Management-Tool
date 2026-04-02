use crate::{
    models::announcements::{self, Announcements},
    storage::{self, file_storage::Storage},
};

pub struct AnnouncementService;

impl AnnouncementService {
    pub fn add(storage: &Storage, title: String, message: String) -> Result<String, String> {
        let mut announcements = storage.load_announcements();
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        let id = storage.get_next_id(&announcements);

        let announcement = Announcements::new(id, title, message);
        announcements.push(announcement);

        Ok("Announcement added successfully".to_string())
    }

    pub fn show_all(storage: &Storage) -> Vec<Announcements> {
        storage.load_announcements()
    }

    pub fn delete(storage: &Storage, announcement_id: i64) -> Result<String, String> {
        let mut anouncements = storage.load_announcements();
        let initial_length = anouncements.len();
        anouncements.retain(|a| a.get_id() != announcement_id);

        if anouncements.len() == initial_length {
            return Err("Announcement not found".to_string());
        }

        storage.save_announcements(&anouncements);

        Ok("Announcement deleteed successfully".to_string())
    }
}
