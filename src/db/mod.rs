use postgres;
use postgres::{Client, NoTls};
use lazy_static;
use std::sync::Mutex;
use std::collections::VecDeque;

pub type Result<T> = std::result::Result<T, postgres::error::Error>;


fn connect() -> Client {
    Client::connect("host=localhost user=postgres", NoTls)
                     .expect("Couldn't connect to the database")
}

lazy_static! {
    static ref CONN_POOL: Mutex<VecDeque<Client>> = Mutex::new(VecDeque::new());
}


fn get_client() -> Client {
    let mut pool = CONN_POOL.lock().unwrap();
    if pool.is_empty() {
        return connect();
    }
    pool.pop_front().expect("Failed to pop from the connection pool")
}

fn release_client(c: Client) {
    let mut pool = CONN_POOL.lock().unwrap();
    pool.push_back(c);
}

pub fn with<T>(f: fn(&mut Client) -> T) -> T {
    let mut c = get_client();
    let val = f(&mut c);
    release_client(c);
    return val;
}


/// Wrapper around postgres's query function
pub fn query<T: ?Sized>(
    query: &T,
    params: &[&(dyn postgres::types::ToSql + Sync)]
) -> Result<Vec<postgres::Row>>
where T: postgres::ToStatement
{
    let mut c = get_client();

    let val = c.query(query, params);
    release_client(c);
    return val;
}
