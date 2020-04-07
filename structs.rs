#![allow(dead_code)]

pub enum UserType {
    Driver,
    Owner,
}

pub struct User {
    id: u64,
    pub email: String,
    pub pass_hash: String,
    pub acct_type: UserType,
}

pub struct Lot {
    id: u64,
    owner_id: u64,
    pub name: String,
    pub address: String,
    pub pricing: u64, // whatever; let's say this is cents per minute
}

pub struct Spot {
    id: u64,
    lot_id: u64,
    pub name: String,
}

pub struct Booking {
    id: u64,
    //redundant due to vehicle_id
    //driver_id: u64,
    spot_id: u64,
    vehicle_id: u64,
    // minutes since midnight
    pub start_time: u16, 
    pub end_time: u16,
    // days since 1970.
    pub start_date: u32, // if you're still using this in 2149, you have bigger problems
    // days since start_date
    // 0 for one-time reservations
    pub duration: u32, // if you need a 180-year reservation, you have bigger problems
    pub recurring_weekdays: [bool; 7],
}

pub struct Vehicle {
    id: u64,
    driver_id: u64,
    pub license_plate: String,
    pub name: String,
}

pub struct Report {
    id: u64,
    pub reporting_user: u64,
    pub offending_user: u64,
    pub report_info: String,
}

pub struct PaymentMethod {
    id: u64,
    driver_id: u64,
    pub card_number: String,
}

fn main() {
    println!("just here so rustc doesn't complain");
}
