use crate::models::announcements::Announcements;

pub struct AnnouncementService {
    anouncements: Vec<Announcements>,
    next_id: i64,
}

impl AnnouncementService {
    pub fn new() -> Self {
        Self {
            anouncements: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String, message: String) {
        let announcement = Announcements::new(self.next_id, title, message);

        self.anouncements.push(announcement);
        self.next_id += 1;
    }

    pub fn show_all(&self) -> &[Announcements] {
        &self.anouncements
    }

    pub fn delete(&mut self, announcement_id: i64) -> bool {
        let initial_length = self.anouncements.len();
        self.anouncements.retain(|a| a.get_id() != announcement_id);

        initial_length != self.anouncements.len()
    }
}
