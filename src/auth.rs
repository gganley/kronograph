extern crate rocket;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
// use rocket_contrib::Json;

// All taken from the rocket.rs docs
use crate::types::{ApiStore, ApiKey};

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key = keys[0];

        let db = rocket::State::<ApiStore>::from_request(request).unwrap();

        if db.contains_key(&key.to_string()) {
            Outcome::Success(ApiKey(key.to_string()))
        } else {
            Outcome::Forward(())
        }
    }
}
