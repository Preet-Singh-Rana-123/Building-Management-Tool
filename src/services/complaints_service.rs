use crate::{
    enums::{Priority, Status},
    models::{complaint::Complaints, resident::Resident},
    storage::file_storage::Storage,
};

pub struct ComplaintService;

//pub struct Complaints {
//    id: i64,
//    title: String,
//    discription: String,
//    status: Status,
//    priority: Priority,
//    resident_id: i64,
//}

impl ComplaintService {
    pub fn add(
        storage: &Storage,
        title: String,
        discription: String,
        status: Status,
        priority: Priority,
        resident: &Resident,
    ) -> Result<String, String> {
        let mut complaints = storage.load_complaints();

        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        let id = storage.get_next_id(&complaints);

        let complaint =
            Complaints::new(id, title, discription, status, priority, resident.get_id());
        complaints.push(complaint);
        storage.save_complaints(&complaints);

        Ok("Complaint added successfully".to_string())
    }

    pub fn show_all(storage: &Storage) -> Vec<Complaints> {
        storage.load_complaints()
    }

    pub fn update_status(
        storage: &Storage,
        complaint_id: i64,
        new_status: Status,
    ) -> Result<String, String> {
        let mut complaints = storage.load_complaints();

        for complaint in &mut complaints {
            if complaint.get_id() == complaint_id {
                complaint.update_status(new_status);
                storage.save_complaints(&complaints);
                return Ok("Status updated successfully".to_string());
            }
        }

        Err("Complaint not found".to_string())
    }

    pub fn show_by_status(storage: &Storage, status: Status) -> Vec<Complaints> {
        let complaints = storage.load_complaints();
        complaints
            .into_iter()
            .filter(|c| c.get_status() == status)
            .collect()
    }

    pub fn show_by_resident_id(storage: &Storage, resident_id: i64) -> Vec<Complaints> {
        let complaints = storage.load_complaints();
        complaints
            .into_iter()
            .filter(|c| c.get_resident_id() == resident_id)
            .collect()
    }
}
