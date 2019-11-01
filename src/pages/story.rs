use rocket::State;
use crate::DbConn;
use rusqlite::Error;


#[post("/story")]
pub fn create_story(db_conn: State<DbConn>) -> Result<String, Error> {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT username FROM User",
                   &[]: &[&str; 0],
                   |row| { row.get(0) })
}

#[get("/story/<id>")]
pub fn story(db_conn: State<DbConn>, id: usize) -> Result<String, Error> {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT username FROM User",
                   &[]: &[&str; 0],
                   |row| { row.get(0) })
}

#[post("/story/<id>/update")]
pub fn update_wordcount(db_conn: State<DbConn>, id: usize) -> Result<String, Error> {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT username FROM User",
                   &[]: &[&str; 0],
                   |row| { row.get(0) })
}