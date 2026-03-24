use crate::enums::{Status, Priority};

pub struct Complaints{
    id: i64,
    title: String,
    discription: String,
    status: Status,
    priority: Priority,
    resident_id: i64,
}
