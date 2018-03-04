

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

//Note we're gonna need to add some concurrency blocking here to avoid SQLITE shitting itself 
pub  fn get_database_connection() -> SqliteConnection {
    let database_url = "data.sqlite";
    SqliteConnection::establish(database_url).expect("Tried to get url")
}

