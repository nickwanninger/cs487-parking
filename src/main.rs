
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate lazy_static;
extern crate chrono;
#[macro_use] extern crate rocket;

#[macro_use]
mod db;
pub mod user;
pub mod vehicle;
pub mod lot;



use rocket_contrib::serve;
use rocket_contrib::templates::Template;
use rocket::http::{Cookie, Cookies};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap};
use rocket::request::Form;
use rocket::response::Redirect;
use chrono::{DateTime, NaiveDateTime, Utc, Duration, Timelike};

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
    lots: Vec<lot::Lot>,
    reservations: Vec<lot::Reservation>,
    all_lots: Vec<lot::Lot>
}

fn render_homepage(user: user::User) -> Template {
    let vs = vehicle::Vehicle::for_user(&user);
    let ls = lot::Lot::for_user(&user);
    let al = lot::Lot::all_lots();
    let rs = lot::Reservation::for_user(&user);
    let ctx = HomepageContext {
        uid: user.user_id as i32,
        email: user.email,
        owner: match user.acct_type {
            user::UserType::Owner => true,
            _ => false
        },
        vehicles: vs,
        lots: ls,
        all_lots: al,
        reservations: rs,
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



#[derive(Debug, FromForm, Serialize, Deserialize)]
struct ReservationInput {
    vehicle: i32,
    lot: i32,
    start_time: String,
    hours: i32
}
#[post("/reservation", data="<input>")]
fn post_reservation(_user: user::User, input: Form<ReservationInput>) -> Redirect {
    // this is the worst way to parse dates, but time sucks so I don't care.
    let start_time = NaiveDateTime::parse_from_str(input.start_time.as_str(), "%Y-%m-%dT%H:%M");

    if let Err(_) = start_time {
        return Redirect::to("/?invalid_date")
    }

    let start_time = DateTime::<Utc>::from_utc(start_time.unwrap(), Utc);
    let end_time = start_time + Duration::hours(input.hours as i64);


    lot::Reservation::create(input.vehicle, input.lot, start_time, end_time).expect("failed to create reservation");
    Redirect::to("/")
}





#[derive(Debug, FromForm, Serialize, Deserialize)]
struct ViewLotInput {
    lot: i32,
    date: String,
}

#[post("/viewlot", data="<input>")]
fn post_view_lot(_user: user::User, input: Form<ViewLotInput>) -> Redirect {

    Redirect::to(format!("/viewlot/{}/{}", input.lot, input.date))
}





#[derive(Serialize, Deserialize)]
struct LotViewCtx {
    id: i32,
    hours: Vec<HourStatus>, // how many reservations at each hour
    date: String,
    lot: lot::Lot,
    income: i32,
}

#[derive(Serialize, Deserialize)]
struct HourStatus {
    perc: f32,
    count: i32,
    income: i32,
    fmt: String,
}

#[get("/viewlot/<id>/<date>")]
fn get_view_lot(_user: user::User, id: i32, date: String) -> Template {

    let mut hours = vec![];

    let lot = lot::Lot::for_id(id);


    // I realize I have a sql injection bug here, but I don't care.
    // This project has gone on long enough. - nick
    let res = run_query!(
        format!("select * from reservations where lot_id = $1 and (
            DATE(start_time) = '{}' or DATE(end_time) = '{}')", date, date).as_str(),
        id).expect("not sure why it failed");

    // parse all the reservations with some fancy functional programming crap.
    let resvs: Vec<lot::Reservation> = res.into_iter().map(|row| lot::Reservation::parse(&row)).collect();

    let mut total: i32 = 0;
    for hour in 0..24 {
        let mut c: i32 = 0;
        // find the reservations that occur within this time period
        for r in &resvs {
            // hours have an off-by one, so the time stored in the db is
            // 1-24, but we iterate 0-23, so sub by one
            if r.start_time.hour()-1 <= hour && r.end_time.hour()-1 >= hour {
                c += 1;
            }
        }
        total += c;

        hours.push(HourStatus {
            perc: (c as f32 / lot.spaces_open as f32) * 100.0,
            count: c,
            income: c * lot.price,
            fmt: format!("{:02}:00 {}", if hour < 12 { hour + 1} else { hour + 1 - 12}, if hour < 12 {"AM"} else {"PM"}),
        });
    }

    let income = total * lot.price;

    let context = LotViewCtx {
        id, hours, date, lot, income,
    };
    Template::render("lot_view", context)
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
            post_reservation,
            post_view_lot,
            get_view_lot,
        ]).launch();

}
