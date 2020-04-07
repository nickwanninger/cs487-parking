use db;

pub struct Lot {
    id: u64,
    owner_id: u64,
    pub name: String,
    pub address: String,
    pub pricing: u64, // whatever; let's say this is cents per minute
}


impl Lot {
    /// Get a list of all slots
    pub fn get_slots(&self) -> db::Result<[Spot]> {
        unimplemented!()
    }

    /// Add a spot to this parking lot and return it's ID
    pub fn add_spot(&mut self, String name) -> db::Result<u64> {
        let sp = Spot { id: 0
    }

    /// Save the lot to the database
    pub fn save(&self) -> Result<(), ()> {
        unimplemented!()
    }
}


/// A single parking spot in a lot
pub struct Spot {
    id: u64,
    lot_id: u64,
    pub name: String,
}

impl Spot {
    /// Delete a spot from the database
    fn delete(id: u64) -> db::Result<()> {
        unimplemented!()
    }



    /// Delete a spot from the database

}
