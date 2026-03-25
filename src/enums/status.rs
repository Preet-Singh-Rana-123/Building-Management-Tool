#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}
