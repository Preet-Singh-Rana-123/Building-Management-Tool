use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}
