
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod db;
pub mod user;


#[macro_use] extern crate rocket;

use rocket_contrib::serve;
use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap};
use rocket::request::Form;
use rocket::response::Redirect;

/// the api needs a context, but we might not need one. It's annoying to
/// create it each time.
fn just_render(name: &'static str) -> Template {
    let context = HashMap::<String, String>::new();
    Template::render(name, context)
}


#[get("/login")]
fn get_login() -> Template {
    just_render("login")
}


#[derive(FromForm, Serialize, Deserialize)]
struct LoginInput {
    email: String,
    password: String,
}
#[post("/login", data = "<input>")]
fn post_login(input: Form<LoginInput>, mut cookies: Cookies) -> Redirect {
    match user::User::login(&input.email, &input.password) {
        Ok(u) => {
            let cookie = Cookie::build("user_id", format!("{}", u.id))
                                 .path("/")
                                 .finish();

            cookies.add(cookie);

            Redirect::to(format!("/?id={}", u.id))
        },
        _ => Redirect::to(format!("/?failed")),
    }
}


#[derive(FromForm, Serialize, Deserialize)]
struct SignupInput {
    email: String,
    password: String,
    atype: String,
}

#[post("/signup", data = "<input>")]
fn post_signup(input: Form<SignupInput>, mut cookies: Cookies) -> Redirect {

    let t = match input.atype.as_str() {
        "parker" => user::UserType::Parker,
        "owner" => user::UserType::Owner,
        _ => return Redirect::to("/?what")
    };
    match user::User::signup(&input.email, &input.password, t) {
        Ok(u) => {
            let cookie = Cookie::build("user_id", format!("{}", u.id))
                                 .path("/")
                                 .finish();
            cookies.add(cookie);

            Redirect::to("/")
        },
        _ => Redirect::to(format!("/?user_exists")),
    }
}


fn render_parker_home(user: &user::User) -> Template {
    Template::render("parker", user)
}


fn render_owner_home(user: &user::User) -> Template {
    Template::render("owner", user)
}



#[get("/", rank=2)]
fn index(user: Option<user::User>) -> Template {
    match user {
        None => just_render("index"),
        Some(user) => {
            match user.acct_type {
                user::UserType::Parker => render_parker_home(&user),
                user::UserType::Owner => render_owner_home(&user)
            }
        }
    }
}


#[get("/logout")]
fn logout(_user: user::User, mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("user_id"));
    Redirect::to("/?logged_out")
}


#[get("/me.json")]
fn me_json(user: user::User) -> String {
    serde_json::to_string(&user).unwrap()
}


fn main() {
    rocket::ignite()
        .mount("/static", serve::StaticFiles::from("static"))
        .attach(Template::fairing())
        .mount("/", routes![index, get_login, post_login, me_json, logout, post_signup]).launch();

}
