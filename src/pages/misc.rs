use rocket::{State, Request};
use crate::DbConn;
use rusqlite::Error;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::response::Redirect;
use crate::pages::login::User;

//#[get("/")]
//pub fn index(db_conn: State<DbConn>) -> Result<String, Error>  {
//    db_conn.lock()
//        .expect("db connection lock")
//        .query_row("SELECT username FROM User",
//                   &[]: &[&str;0],
//                   |row| { row.get(0) })
//}

#[get("/")]
pub fn user_index(user: User) -> Template {
    let mut context = HashMap::new();
    context.insert("user_id", user.0);
    Template::render("index", &context)
}

#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(crate::pages::login::login_page))
}




#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}
