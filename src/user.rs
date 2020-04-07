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

impl User {
    /// Save the user to the database
    pub fn save(&self) -> Result<(), ()> {
        unimplemented!()
    }

    /// Lookup a user by id in the database
    pub fn lookup(id: u64) -> Result<User, ()> {
        unimplemented!()
    }
}
