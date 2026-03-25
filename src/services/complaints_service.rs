use crate::{
    enums::{Priority, Status},
    models::{complaint::Complaints, resident::Resident},
};

pub struct ComplaintService {
    complaints: Vec<Complaints>,
    next_id: i64,
}
//pub struct Complaints {
//    id: i64,
//    title: String,
//    discription: String,
//    status: Status,
//    priority: Priority,
//    resident_id: i64,
//}

impl ComplaintService {
    pub fn new() -> Self {
        Self {
            complaints: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(
        &mut self,
        title: String,
        discription: String,
        status: Status,
        priority: Priority,
        resident: &Resident,
    ) {
        let complaint = Complaints::new(
            self.next_id,
            title,
            discription,
            status,
            priority,
            resident.get_id(),
        );

        self.complaints.push(complaint);
        self.next_id += 1;
    }

    pub fn show_all(&self) -> &[Complaints] {
        &self.complaints
    }

    pub fn update_status(&mut self, complaint_id: i64, new_status: Status) -> bool {
        if let Some(complaint) = self
            .complaints
            .iter_mut()
            .find(|complaint| complaint.get_id() == complaint_id)
        {
            complaint.update_status(new_status);
            return true;
        }

        false
    }

    pub fn show_by_status(&self, status: Status) -> Vec<&Complaints> {
        self.complaints
            .iter()
            .filter(|c| c.get_status() == status)
            .collect()
    }
}
