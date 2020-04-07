
#[macro_use]
extern crate lazy_static;

pub mod db;

fn main() {
    let mut c = db::Connection::new();

    let rows = c.query(
        "select table_name from information_schema.tables;",
        &[]).expect("failed");

    println!("got back {} rows", rows.len());
}
