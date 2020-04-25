use crate::db;

extern crate bcrypt;
use std::result::{Result};
use rocket::Request;
use rocket::request::{self, FromRequest};
use crate::rocket::outcome::IntoOutcome;
use serde::{Serialize, Deserialize};

use postgres;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {
    Parker,
    Owner,
}


impl UserType {
    pub fn to_db(&self) -> i32 {
        match self {
            UserType::Parker => 0,
            UserType::Owner => 1
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub pass_hash: String,
    pub acct_type: UserType,
}

impl User {
    fn parse(row: &postgres::Row) -> User {
        let t: i32 = row.get("acct_type");
        let id: i32 = row.get("user_id");
        User {
            user_id: id,
            email: row.get("email"),
            pass_hash: row.get("pass_hash"),
            acct_type: match t {
                0 => UserType::Parker,
                1 => UserType::Owner,
                _ => panic!("invalid account type")
            },
        }
    }

    /// Create a user in the database and return it
    pub fn create(email: String, phash: String, acct_type: UserType) -> db::Result<User> {
        let atype = acct_type.to_db();

        let res = run_query!("INSERT INTO users(email, pass_hash, acct_type)
                              VALUES
                                  ($1, $2, $3)
                              RETURNING *;",
                email, phash, atype)?;

        Ok(User::parse(&res[0]))
    }


    /// Lookup and return the user id of a user if login was successfull
    pub fn login(email: &String, password: &String) -> Result<User, ()> {
        let res = run_query!("select * from users where email = $1;", email);
        if let Ok(res) = res {
            let u = User::parse(&res[0]);
            if u.verify_password(password) {
                return Ok(u);
            }
        }

        Err(())
    }


    pub fn signup(email: &String, password: &String, t: UserType) -> Result<User, ()> {

        let password = hash_password(&password);
        let res = run_query!("select * from users where email = $1;", email);

        if let Ok(res) = res {
            if res.len() != 0 {
                println!("user exists");
                return Err(());
            }
        }

        match User::create(email.to_string(), password, t) {
            Ok(u) => Ok(u),
            Err(_) => Err(())
        }
    }



    /// Save the user to the database
    pub fn save(&self) -> Result<(), ()> {
        unimplemented!()
    }



    /// Lookup a user by id in the database
    pub fn lookup(id: i32) -> Option<User> {
        let res = run_query!("select * from users where id = $1;", id);

        if let Ok(res) = res {
            return Some(User::parse(&res[0]));
        }

        None
    }

    pub fn verify_password(&self, password: &String) -> bool {
        bcrypt::verify(password, &self.pass_hash).expect("failed to verify")
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        request.cookies()
            .get("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User::lookup(id).unwrap())
            .or_forward(())
    }
}


pub fn hash_password(pw: &String) -> String {
    bcrypt::hash(pw, 6).expect("failed to hash")
}
