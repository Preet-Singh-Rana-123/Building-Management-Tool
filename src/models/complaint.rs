use crate::enums::{Priority, Status};

pub struct Complaints {
    id: i64,
    title: String,
    discription: String,
    status: Status,
    priority: Priority,
    resident_id: i64,
}

impl Complaints {
    pub fn new(
        id: i64,
        title: String,
        discription: String,
        status: Status,
        priority: Priority,
        resident_id: i64,
    ) -> Self {
        Self {
            id,
            title,
            discription,
            status,
            priority,
            resident_id,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }
}
