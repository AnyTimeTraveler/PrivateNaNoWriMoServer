use rocket::State;
use crate::DbConn;
use rusqlite::Error;

#[get("/user/<name>")]
pub fn user(db_conn: State<DbConn>, name: String) -> Result<String, Error> {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT username FROM User",
                   &[]: &[&str; 0],
                   |row| { row.get(0) })
}
