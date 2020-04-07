
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod db;
pub mod user;


fn main() {

    let u = user::User::create(
                    String::from("my@email"),
                    String::from("pw"),
                    user::UserType::Owner
                ).expect("failed to create user");

    println!("user: {:#?}", u);

}
