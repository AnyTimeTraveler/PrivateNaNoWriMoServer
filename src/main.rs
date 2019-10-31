#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use] extern crate rocket;
extern crate rusqlite;


use std::sync::Mutex;
use rocket::{Rocket, State};
use rusqlite::{Connection, Error};
use std::fs::File;
use std::io::Read;

type DbConn = Mutex<Connection>;

fn init_database(conn: &Connection) {
    let mut sql_init = File::open("res/init.sql").expect("opening sql init file");
    let mut sql = String::new();
    sql_init.read_to_string(&mut sql).expect("reading sql init file");
    conn.execute_batch(&*sql).expect("create entries table");
}

#[get("/")]
fn hello(db_conn: State<DbConn>) -> Result<String, Error>  {
    db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT username FROM User",
                   &[]: &[&str;0],
                   |row| { row.get(0) })
}

fn rocket() -> Rocket {
    // Open a new in-memory SQLite database.
    let conn = Connection::open("database.sqlite").expect("in memory db");

    // Initialize the `entries` table in the in-memory database.
    init_database(&conn);

    // Have Rocket manage the database pool.
    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![hello])
}

fn main() {
    rocket().launch();
}
