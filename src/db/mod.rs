use postgres;
use postgres::{Client, NoTls};
use lazy_static;
use std::sync::{Mutex, Arc};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};



pub type Result<T> = std::result::Result<T, postgres::error::Error>;


fn connect() -> Arc<Client> {
    Arc::new(Client::connect("host=localhost user=postgres dbname=parking", NoTls)
                     .expect("Couldn't connect to the database"))
}

lazy_static! {
    static ref CONN_POOL: Mutex<VecDeque<Arc<Client>>> = Mutex::new(VecDeque::new());
}


pub fn get_client() -> Arc<Client> {
    let mut pool = CONN_POOL.lock().unwrap();
    if pool.is_empty() {
        return connect();
    }
    pool.pop_front().expect("Failed to pop from the connection pool")
}

pub fn drop_client(c: Arc<Client>) {
    let mut pool = CONN_POOL.lock().unwrap();
    pool.push_back(c);

    println!("dropped client: {} connected", pool.len());
}

#[macro_export]
macro_rules! run_query {
    ( $q:expr ) => {
        db::Connection::new().query($q, &[])
    };
    ( $q:expr, $( $y:expr ),* ) => {
        db::Connection::new().query($q, &[$( &$y ),*])
    };
}


/// An owned reference to a client, returning the client to the pool
/// on dropping
///
/// Example:
/// ```
/// let mut c = db::Connection::new();
/// c.query(...)
/// // drops!
/// ```
pub struct Connection {
    c: Arc<Client>
}


impl Connection {
    /// Construct a new connection instance, grabbing a
    /// client using RAII.
    pub fn new() -> Connection {
        Connection {
            c: get_client()
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        drop_client(self.c.clone());
    }
}


impl Deref for Connection {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Client {
        Arc::get_mut(&mut self.c).expect("oh no")
    }
}
