use crate::ui::input::{self, get_input};

pub fn auth_page() -> i32 {
    println!("=============================");
    println!("Building Manager System");
    println!("=============================");
    println!();
    println!("1. Login");
    println!("2. Register (Resident)");
    println!("3. Exit");

    get_input("Enter Choice: ").parse().unwrap_or(0)
}

pub fn owner_dashboard() -> i32 {
    println!("=============================");
    println!("OWNER DASHBOARD");
    println!("=============================");
    println!();

    println!("1. Manage Residents");
    println!("2. Manage Complaints");
    println!("3. Announcements");
    println!("4. View Logs");
    println!("5. Logout");

    get_input("Enter Choice: ").parse().unwrap_or(0)
}

pub fn resident_mangement_dashboard() -> i32 {
    println!("==== RESIDENT MANAGEMENT ====");
    println!(
        "1. Add Resident
2. View All Residents
3. Search Resident
4. Update Resident
5. Delete Resident
6. Back"
    );

    get_input("Enter Choice: ").parse().unwrap_or(0)
}

pub fn resident_search() -> i32 {
    println!(
        "Search by:
1. ID
2. Flat Number"
    );

    get_input("Enter Choice: ").parse().unwrap_or(0)
}

pub fn resident_update() -> i32 {
    println!("Enter Resident ID:");
    println!("1. Update Name");
    println!("2. Update Phone");
    println!("3. Update Flat");

    get_input("Enter Choice: ").parse().unwrap_or(0)
}

pub fn complaint_management_dashboard() -> i32 {
    println!("==== COMPLAINT MANAGEMENT ====");
    println!("");
    println!("1. View All Complaints");
    println!("2. Filter by Status");
    println!("3. Update Status");
    println!("4. Assign Priority");
    println!("5. Back ");

    get_input("Enter Choice: ").parse().unwrap_or(0)
}


