use chrono::{DateTime, Local};

pub struct Announcements{
    id: i64,
    title: String,
    message: String,
    created_at: DateTime<Local>,
}
