#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};
use rocket::Rocket;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    text: String,
}

impl<'r> Responder<'r> for Message {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
          let json_str = serde_json::to_string_pretty(&self).unwrap();
          Response::build()
            .status(Status::Ok)
            .sized_body(Cursor::new(json_str))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[get("/")]
fn index() -> Message {
    return Message {
        text: "Hello world".to_owned(),
    };
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
