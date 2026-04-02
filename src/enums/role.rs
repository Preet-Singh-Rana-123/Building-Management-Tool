use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Role{
    Owner,
    Resident,
}
