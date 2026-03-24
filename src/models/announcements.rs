use chrono::{DateTime, Local};

pub struct Announcements {
    id: i64,
    title: String,
    message: String,
    created_at: DateTime<Local>,
}

impl Announcements {
    pub fn new(id: i64, title: String, message: String) -> Self {
        Self {
            id,
            title,
            message,
            created_at: Local::now(),
        }
    }
}
