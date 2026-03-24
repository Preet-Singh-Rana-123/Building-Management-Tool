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

    pub fn show_details(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Flat: {}", self.flat_number);
        println!("Phone No.: {}", self.phone);
    }
}
