

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use std::sync::Mutex;
use lazy_static;



//Note we're gonna need to add some concurrency blocking here to avoid SQLITE shitting itself 
pub  fn get_database_connection() -> SqliteConnection {
    SqliteConnection::establish("data.sqlite").unwrap()
}

