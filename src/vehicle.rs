use crate::db;
use crate::user;


use serde::{Serialize, Deserialize};


/// Represents a single instance of a driver's vehicle
#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub vehicle_id: i32,
    pub driver_id: i32,
    pub license_plate: String,
    pub name: String,
}


impl Vehicle {
    pub fn parse(row: &postgres::Row) -> Vehicle {
        Vehicle {
            vehicle_id: row.get("vehicle_id"),
            driver_id: row.get("driver_id"),
            license_plate: row.get("license_plate"),
            name: row.get("name")
        }
    }


    pub fn create(user: user::User, license: String, name: String) -> db::Result<Vehicle> {
        let res = run_query!("INSERT INTO vehicles(driver_id, license_plate, name)
                              VALUES ($1, $2, $3)
                              RETURNING *;",
                user.user_id, license, name)?;

        Ok(Vehicle::parse(&res[0]))
    }


    pub fn for_user(user: &user::User) -> Vec<Vehicle> {
        let res = run_query!("SELECT * FROM VEHICLES WHERE driver_id = $1;",
                user.user_id);

        let mut v = vec![];

        if let Ok(res) = res {
            for row in res {
                v.push(Vehicle::parse(&row));
            }
        }
        return v;
    }


    pub fn delete(id: i32) {
        run_query!("DELETE FROM VEHICLES WHERE vehicle_id = $1;", id).expect("failed to delete vehicle");

        // TODO: update reservations and parking lot statuses
    }


}
