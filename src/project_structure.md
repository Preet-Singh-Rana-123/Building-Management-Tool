# Building Manager Project Structure

```
building_manager/
│
├── Cargo.toml
├── Cargo.lock
│
├── src/
│ ├── main.rs # Entry point
│ │
│ ├── app.rs # App controller (navigation + flow)
│ │
│ ├── models/ # Data structures
│ │ ├── mod.rs
│ │ ├── user.rs
│ │ ├── resident.rs
│ │ ├── complaint.rs
│ │ ├── announcement.rs
│ │
│ ├── services/ # Business logic
│ │ ├── mod.rs
│ │ ├── auth_service.rs
│ │ ├── resident_service.rs
│ │ ├── complaint_service.rs
│ │ ├── announcement_service.rs
│ │
│ ├── storage/ # Data persistence
│ │ ├── mod.rs
│ │ ├── file_storage.rs # JSON-based storage
│ │ ├── db.rs # (future: SQLite/Postgres)
│ │
│ ├── ui/ # CLI interface (later → TUI)
│ │ ├── mod.rs
│ │ ├── menu.rs
│ │ ├── input.rs
│ │ ├── output.rs
│ │
│ ├── utils/ # Helpers
│ │ ├── mod.rs
│ │ ├── validator.rs
│ │ ├── logger.rs
│ │ ├── id_generator.rs
│ │
│ ├── enums/ # Rust enums (important)
│ │ ├── mod.rs
│ │ ├── role.rs
│ │ ├── status.rs
│ │ ├── priority.rs
│ │
│ ├── errors/ # Custom error handling
│ │ ├── mod.rs
│ │ ├── app_error.rs
│
├── data/ # Local storage (JSON files)
│ ├── users.json
│ ├── complaints.json
│ ├── announcements.json
│
├── logs/
│ ├── app.log
│
└── README.md
```
