use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,PartialEq, Debug, Clone, Copy, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}
