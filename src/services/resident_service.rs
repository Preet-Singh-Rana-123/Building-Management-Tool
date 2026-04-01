use crate::models::resident::Resident;
use crate::storage::file_storage::Storage;

pub struct ResidentService;

impl ResidentService {
    pub fn add(
        storage: &Storage,
        name: String,
        flat: String,
        phone: String,
    ) -> Result<String, String> {
        let mut residents = storage.load_residents();

        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let id = storage.get_next_id(&residents);

        let resident = Resident::new(id, name, flat, phone);
        residents.push(resident);

        storage.save_residents(&residents);

        Ok("Resident added successfully".to_string())
    }

    pub fn show_all(storage: &Storage) -> Vec<Resident> {
        storage.load_residents()
    }

    pub fn show_specific(storage: &Storage, resident_id: i64) -> Option<Resident> {
        let residents = storage.load_residents();

        residents.into_iter().find(|r| r.get_id() == resident_id)
    }

    pub fn update_name(
        storage: &Storage,
        resident_id: i64,
        new_name: String,
    ) -> Result<String, String> {
        let mut residents = storage.load_residents();

        for resident in &mut residents {
            if resident_id == resident.get_id() {
                resident.update_name(new_name);
                storage.save_residents(&residents);
                return Ok("Name Updated successfully".to_string());
            }
        }

        Err("Resident not found".to_string())
    }

    pub fn update_phone(
        storage: &Storage,
        resident_id: i64,
        new_phone: String,
    ) -> Result<String, String> {
        let mut residents = storage.load_residents();

        for resident in &mut residents {
            if resident_id == resident.get_id() {
                resident.update_phone(new_phone);
                storage.save_residents(&residents);
                return Ok("Flat Updated successfully".to_string());
            }
        }

        Err("Resident not found".to_string())
    }

    pub fn update_flat(
        storage: &Storage,
        resident_id: i64,
        new_flat: String,
    ) -> Result<String, String> {
        let mut residents = storage.load_residents();

        for resident in &mut residents {
            if resident_id == resident.get_id() {
                resident.update_flat(new_flat);
                storage.save_residents(&residents);
                return Ok("Flat Updated successfully".to_string());
            }
        }

        Err("Resident not found".to_string())
    }

    pub fn delete(storage: &Storage, resident_id: i64) -> Result<String, String> {
        let mut residents = storage.load_residents();

        let initial_length = residents.len();

        residents.retain(|r| r.get_id() != resident_id);

        if residents.len() == initial_length {
            return Err("Resident not found".to_string());
        }

        storage.save_residents(&residents);

        Ok("Resident deleted successfully".to_string())
    }
}
