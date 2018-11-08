#![feature(proc_macro_hygiene, decl_macro)]

use rocket;
use rocket::{get};


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
        .mount("/", rocket::routes![index])
        .mount(
            "/task",
            rocket::routes![entry::add, entry::edit, entry::delete, entry::backfill],
        )
        .mount(
            "/project",
            rocket::routes![
                project::add,
                project::edit,
                project::delete,
                project::data
            ],
        )
        .mount(
            "/user",
            rocket::routes![user::add, user::delete, user::information],
        )
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};
    
    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome to Kronograph!".into()));
    }


    /// This tests `user::add`
    #[test]
    fn user_add() {
        let user_object = serde_json::json!(
            {
                "email": "me@gganley.com",
                "secret": "testPass!"
            }
        );
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.post("/user/add")
            .header(ContentType::JSON)
            .body(user_object.to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        match response.body_string() {
            Some(x) => {
                // TODO: Also ensure that the apikey is present at the very least
                let v: serde_json::Value = serde_json::from_str(&x).unwrap();
                assert_eq!(v["email"], "me@gganley.com".to_string())
            },
            None => panic!("Could not retrieve body string. \n\nRESPONSE\n{:?}\n", response)
        }
    }

    #[test]
    fn entry_add() {
        
    }

    #[test]
    fn entry_info() {
        
    }

    #[test]
    fn entry_remove() {
        
    }
}

