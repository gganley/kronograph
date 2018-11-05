#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod auth;
mod entry;
mod project;
mod types;
mod user;

use crate::types::ApiStore;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Kronograph!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(ApiStore::default())
        .mount("/", routes![index])
        .mount(
            "/task",
            routes![entry::add, entry::edit, entry::delete, entry::backfill],
        )
        .mount(
            "/project",
            routes![
                project::create,
                project::edit,
                project::delete,
                project::data
            ],
        )
        .mount(
            "/user",
            routes![user::create, user::delete, user::information],
        )
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;
    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome to Kronograph!".into()));
    }
}
