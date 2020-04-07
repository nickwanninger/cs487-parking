

/// Represents a single (or recurring) parking reservation
pub struct Booking {
    id: u64,
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

impl Booking {
    /// Returns if two booking objects conflict in any way
    fn conflicts(&self, _other: &Booking) -> bool {
        false;
    }
}


#[test]
fn booking_confliction() {
    // assert_eq!(2 + 3, add(2, 3));
}
