use crate::models::resident::Resident;

pub struct ResidentService {
    residents: Vec<Resident>,
    next_id: i64,
}

impl ResidentService {
    pub fn new() -> Self {
        Self {
            residents: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, name: String, flat: String, phone: String) {
        let resident = Resident::new(self.next_id, name, flat, phone);

        self.residents.push(resident);
        self.next_id += 1;
    }

    pub fn show_all(&self) -> &[Resident] {
        &self.residents
    }

    pub fn show_specific(&self, resident_id: i64) -> Option<&Resident> {
        let result = self
            .residents
            .iter()
            .find(|resident| resident.get_id() == resident_id);
        result
    }

    pub fn update_name(&mut self, resident_id: i64, new_name: String) -> bool {
        for resident in &mut self.residents {
            if resident.get_id() == resident_id {
                resident.update_name(new_name);
                return true;
            }
        }

        false
    }

    pub fn update_phone(&mut self, resident_id: i64, new_phone: String) -> bool {
        for resident in &mut self.residents {
            if resident.get_id() == resident_id {
                resident.update_phone(new_phone);
                return true;
            }
        }

        false
    }

    pub fn update_flat(&mut self, resident_id: i64, new_flat: String) -> bool {
        for resident in &mut self.residents {
            if resident.get_id() == resident_id {
                resident.update_flat(new_flat);
                return true;
            }
        }

        false
    }

    pub fn delete(&mut self, resident_id: i64) -> bool {
        let initial_length = self.residents.len();
        self.residents
            .retain(|resident| resident.get_id() != resident_id);

        initial_length != self.residents.len()
    }
}
