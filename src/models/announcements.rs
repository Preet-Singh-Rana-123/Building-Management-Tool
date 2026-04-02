use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use crate::traits::identifiable::Identifiable;

#[derive(Serialize, Deserialize, Clone)]
pub struct Announcements {
    id: i64,
    title: String,
    message: String,
    created_at: DateTime<Local>,
}

impl Identifiable for Announcements {
    fn get_id(&self) -> i64 {
        self.id
    }
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

    pub fn get_id(&self) -> i64 {
        self.id
    }
}

