use crate::db;
use crate::user;
use crate::vehicle;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

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



    pub fn all_lots() -> Vec<Lot> {
        let res = run_query!("SELECT * FROM lots;").expect("failed to query for all lots");
        let mut v = vec![];
        for row in res {
            v.push(Lot::parse(&row));
        }
        return v;
    }


    pub fn for_id(id: i32) -> Lot {
        let res = run_query!("SELECT * FROM lots where lot_id = $1;", id).expect("failed to query for all lots");
        Lot::parse(&res[0])
    }

}


/// Represents a reservation event
#[derive(Serialize, Deserialize)]
pub struct Reservation {
    pub id: i32,
    pub vehicle: vehicle::Vehicle,
    pub lot: Lot,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub cost: i32, // in dollars
    pub hours: f32, // calculated at ::parse()
    pub human_time: String,
}

impl Reservation {
    /// parse a reservation from a database row
    pub fn parse(row: &postgres::Row) -> Reservation {

        let vid: i32 = row.get("vehicle_id");
        let lid: i32 = row.get("lot_id");

        let v = vehicle::Vehicle::parse(
            &run_query!("SELECT * from vehicles where vehicle_id = $1;",
                       vid).expect("huh.")[0]);


        let l = Lot::parse(
            &run_query!("SELECT * from lots where lot_id = $1;",
                       lid).expect("huh.")[0]);

        let start_time: DateTime<Utc> = row.get("start_time");
        let end_time: DateTime<Utc> = row.get("end_time");
        let seconds = end_time.timestamp() - start_time.timestamp();
        let hours: f32 = seconds as f32 / 3600.0;
        let mut cost = (hours.ceil() * l.price as f32) as i32;
        if cost == 0 {
            cost = l.price;
        }

        let human_time = start_time.format("%a %b %e, %Y - %H:%M %P").to_string();
        Reservation {
            id: row.get("reservation_id"),
            vehicle: v,
            lot: l,
            start_time, end_time, cost, hours,
            human_time,
        }
    }

    /// Add a reservation to the DB and return it
    pub fn create(
        vid: i32,
        lid: i32,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> db::Result<Reservation> {
        let res = run_query!("INSERT INTO reservations
                              (vehicle_id, lot_id, start_time, end_time)
                              VALUES ($1, $2, $3, $4)
                              RETURNING *;",
                vid, lid, start, end)?;


        Ok(Reservation::parse(&res[0]))
    }


    pub fn for_user(user: &user::User) -> Vec<Reservation> {
        let res = match user.acct_type {
            user::UserType::Owner => {
                run_query!("SELECT * FROM lots
                            JOIN reservations on lots.lot_id = reservations.lot_id
                            where owner_id = $1;",
                                  user.user_id).expect("oops")
            },
            user::UserType::Parker => {
                run_query!("SELECT * FROM vehicles
                            JOIN reservations on vehicles.vehicle_id = reservations.vehicle_id
                            where driver_id = $1;",
                                  user.user_id).expect("oops")
            }
        };


        let mut v = vec![];
        for row in res {
            v.push(Reservation::parse(&row));
        }
        return v;
    }
}






/*

use bytes::BytesMut;
use postgres::{FromSql, IsNull, ToSql, Type};
use std::error::Error;


impl<'a> FromSql<'a> for DateTime<Utc> {
    fn from_sql(type_: &Type, raw: &[u8]) -> Result<DateTime<Utc>, Box<dyn Error + Sync + Send>> {
        let naive = NaiveDateTime::from_sql(type_, raw)?;
        Ok(DateTime::from_utc(naive, Utc))
    }

    fn accepts(ty: &postgres::Type) -> bool {
        match *ty {
            $($crate::Type::TIMESTAMPTZ)|+ => true,
            _ => false
        }
    }

    accepts!(TIMESTAMPTZ);
}

impl ToSql for DateTime<Utc> {
    fn to_sql(
        &self,
        type_: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.naive_utc().to_sql(type_, w)
    }

    accepts!(TIMESTAMPTZ);
    to_sql_checked!();
}

*/
