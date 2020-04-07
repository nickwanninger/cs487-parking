
#[macro_use]
extern crate lazy_static;

pub mod db;

fn main() {
    let rows = db::query("SELECT table_schema,table_name FROM information_schema.tables", &[]).expect("failed");

    println!("{:#?}", rows);
}
