pub struct Resident {
    id: i64,
    name: String,
    flat_number: String,
    phone: String,
}

impl Resident {
    pub fn new(id: i64, name: String, flat_number: String, phone: String) -> Self {
        Self {
            id,
            name,
            flat_number,
            phone,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn show_details(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Flat: {}", self.flat_number);
        println!("Phone No.: {}", self.phone);
    }

    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn update_phone(&mut self, new_phone: String) {
        self.phone = new_phone;
    }
    pub fn update_flat(&mut self, new_flat: String) {
        self.flat_number = new_flat;
    }
}
