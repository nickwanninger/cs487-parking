use crate::db;
use crate::user;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Lot {
    pub lot_id: i32,
    pub owner_id: i32,
    pub name: String,
    pub address: String,
    pub price: i32,
    pub spaces_open: i32
}


impl Lot {

    /// parse a lot from a database row
    pub fn parse(row: &postgres::Row) -> Lot {
        Lot {
            lot_id: row.get("lot_id"),
            owner_id: row.get("owner_id"),
            name: row.get("name"),
            address: row.get("address"),
            price: row.get("price"),
            spaces_open: row.get("spaces_open"),
        }
    }


    /// Create a lot owned by a specific user.
    pub fn create(user: user::User,
          name: &String,
          address: &String,
          price: i32
        ) -> db::Result<Lot> {
        let res = run_query!("INSERT INTO lots(owner_id, name, address, price)
                              VALUES ($1, $2, $3, $4)
                              RETURNING *;",
                user.user_id, name, address, price)?;

        Ok(Lot::parse(&res[0]))
    }


    /// Get all the lots for a user
    pub fn for_user(user: &user::User) -> Vec<Lot> {
        let res = run_query!("SELECT * FROM lots WHERE owner_id = $1;",
                user.user_id);

        let mut v = vec![];

        if let Ok(res) = res {
            for row in res {
                v.push(Lot::parse(&row));
            }
        }
        return v;
    }

    pub fn delete(id: i32) {
        run_query!("DELETE FROM lots WHERE lot_id = $1;", id).expect("Failed to delete!");

        // TODO: delete reservations
    }
}
