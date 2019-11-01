#![feature(proc_macro_hygiene, decl_macro, type_ascription, never_type)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rusqlite;

mod pages;

use std::sync::Mutex;
use rocket::Rocket;
use rusqlite::Connection;
use std::fs::File;
use std::io::Read;

type DbConn = Mutex<Connection>;

fn reset_database(conn: &Connection) {
    let mut sql_init = File::open("res/init.sql").expect("opening sql init file");
    let mut sql = String::new();
    sql_init.read_to_string(&mut sql).expect("reading sql init file");
    conn.execute_batch(&*sql).expect("create entries table");
}

fn rocket() -> Rocket {
    let conn = Connection::open("database.sqlite").expect("in memory db");

    // Only while testing
    reset_database(&conn);

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![
            pages::misc::index,
            pages::misc::user_index,
            pages::login::login,
            pages::login::logout,
            pages::login::login_user,
            pages::login::login_page,
            pages::user::user,
            pages::story::create_story,
            pages::story::story,
            pages::story::update_wordcount
        ])
        .register(catchers![pages::misc::not_found])
}

fn main() {
    rocket().launch();
}
