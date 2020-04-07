use postgres;
use postgres::{Client, NoTls};
use lazy_static;
use std::sync::{Mutex, Arc};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};



pub type Result<T> = std::result::Result<T, postgres::error::Error>;


fn connect() -> Arc<Client> {
    Arc::new(Client::connect("host=localhost user=postgres", NoTls)
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
}

/// Wrapper around postgres's query function
pub fn query<T: ?Sized>(
    query: &T,
    params: &[&(dyn postgres::types::ToSql + Sync)]
) -> Result<Vec<postgres::Row>>
where T: postgres::ToStatement
{
    let mut c = get_client();
    let val = Arc::get_mut(&mut c)
            .expect("uh...")
            .query(query, params);
    drop_client(c);
    return val;
}




pub struct Connection {
    c: Arc<Client>
}


impl Connection {
    pub fn new() -> Connection {
        Connection {
            c: get_client()
        }
    }
}



impl Drop for Connection {
    fn drop(&mut self) {
        println!("drop!");
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
