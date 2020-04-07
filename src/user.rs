use crate::db;

#[derive(Debug)]
pub enum UserType {
    Driver,
    Owner,
}

#[derive(Debug)]
pub struct User {
    id: u64,
    pub email: String,
    pub pass_hash: String,
    pub acct_type: UserType,
}

impl User {
    fn parse(row: &postgres::Row) -> User {
        let t: i32 = row.get("acct_type");
        let id: i32 = row.get("id");
        User {
            id: id as u64,
            email: row.get("email"),
            pass_hash: row.get("pass_hash"),
            acct_type: match t {
                0 => UserType::Driver,
                1 => UserType::Owner,
                _ => panic!("invalid account type")
            },
        }
    }

    /// Create a user in the database and return it
    pub fn create(email: String, phash: String, acct_type: UserType) -> db::Result<User> {
        let atype: i32 = match acct_type {
            UserType::Driver => 0,
            UserType::Owner => 1
        };

        let res = run_query!("INSERT INTO users(email, pass_hash, acct_type)
                              VALUES
                                  ($1, $2, $3)
                              RETURNING *;",
                email, phash, atype)?;

        Ok(User::parse(&res[0]))
    }



    /// Save the user to the database
    pub fn save(&self) -> Result<(), ()> {
        unimplemented!()
    }



    /// Lookup a user by id in the database
    pub fn lookup(_id: u64) -> Result<User, ()> {
        unimplemented!()
    }
}
