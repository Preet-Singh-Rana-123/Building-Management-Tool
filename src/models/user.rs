use crate::enums::Role;

struct User {
    id: i64,
    name: String,
    email: String,
    password: String,
    role: Role,
}

impl User {
    pub fn new(id: i64, name: String, email: String, password: String, role: Role) -> Self {
        Self {
            id,
            name,
            email,
            password,
            role,
        }
    }
}
