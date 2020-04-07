
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod db;



fn main() {

    run_query!("INSERT INTO Foo (value)
              VALUES ($1);", 3).expect("failed to query");

    let rows = run_query!("select * from Foo;").expect("failed");

    for row in rows {
        let id: i32 = row.get("id");
        let value: i32 = row.get("value");
        println!("id: {}, value: {}", id, value);
    }
}
