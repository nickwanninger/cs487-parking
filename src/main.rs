
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod db;
pub mod user;
pub mod vehicle;
pub mod lot;


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
            let cookie = Cookie::build("user_id", format!("{}", u.user_id))
                                 .path("/")
                                 .finish();

            cookies.add(cookie);

            Redirect::to(format!("/?id={}", u.user_id))
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
            let cookie = Cookie::build("user_id", format!("{}", u.user_id))
                                 .path("/")
                                 .finish();
            cookies.add(cookie);

            Redirect::to("/")
        },
        _ => Redirect::to(format!("/?user_exists")),
    }
}




#[derive(Serialize, Deserialize)]
struct HomepageContext {
    uid: i32,
    email: String,
    owner: bool,
    vehicles: Vec<vehicle::Vehicle>,
    lots: Vec<lot::Lot>
}

fn render_homepage(user: user::User) -> Template {
    let vs = vehicle::Vehicle::for_user(&user);
    let ls = lot::Lot::for_user(&user);
    let ctx = HomepageContext {
        uid: user.user_id as i32,
        email: user.email,
        owner: match user.acct_type {
            user::UserType::Owner => true,
            _ => false
        },
        vehicles: vs,
        lots: ls
    };
    Template::render("home", ctx)
}



#[get("/", rank=2)]
fn index(user: Option<user::User>) -> Template {
    match user {
        None => just_render("index"),
        Some(user) => render_homepage(user)
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

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct VehiclePostForm {
    license: String,
    name: String,
}

#[post("/vehicle", data="<input>")]
fn post_vehicle(user: user::User, input: Form<VehiclePostForm>) -> Redirect {
    // assume it succeeds
    vehicle::Vehicle::create(user, input.license.to_string(), input.name.to_string()).unwrap();

    Redirect::to("/?added")
}



#[post("/vehicle/delete/<id>")]
fn delete_vehicle(_user: user::User, id: i32) -> Redirect {
    vehicle::Vehicle::delete(id);

    Redirect::to("/")
}



#[derive(Debug, FromForm, Serialize, Deserialize)]
struct LotPostForm {
    name: String,
    address: String,
    price: i32,
}

#[post("/lot", data="<input>")]
fn post_lot(user: user::User, input: Form<LotPostForm>) -> Redirect {
    // assume it succeeds
    lot::Lot::create(user, &input.name, &input.address, input.price).unwrap();

    Redirect::to("/?created_lot")
}




#[post("/lot/delete/<id>")]
fn delete_lot(_user: user::User, id: i32) -> Redirect {
    lot::Lot::delete(id);

    Redirect::to("/")
}

fn main() {
    rocket::ignite()
        .mount("/static", serve::StaticFiles::from("static"))
        .attach(Template::fairing())
        .mount("/", routes![index,
            get_login,
            post_login,
            me_json,
            logout,
            post_signup,
            post_vehicle,
            delete_vehicle,
            post_lot,
            delete_lot,
        ]).launch();

}
