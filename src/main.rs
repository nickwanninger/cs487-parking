
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod db;
pub mod user;


fn main() {

    let u = user::User::create(
                    String::from("my@email"),
                    user::hash_password(String::from("my_password")),
                    user::UserType::Owner
                ).expect("failed to create user");

    println!("user: {:#?}", u);
    loop {
        println!("hash: {:#?}", user::hash_password(String::from("yeet")));
    }

}
