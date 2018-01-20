#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate ws;

use rocket_contrib::Json;
use std::thread;
use ws::listen;

#[derive(Debug, Serialize)]
struct CreateGameResponse {
  id: String
}

#[post("/games")]
fn create_game() -> Json<CreateGameResponse> {
  Json(CreateGameResponse { id: "hello".to_owned() })
}

fn main() {
  thread::spawn(|| {
    listen("127.0.0.1:5000", |out| {
      move |msg| {
        println!("{:?}", msg);
        out.send(msg)
      }
    }).unwrap();
  });

  rocket::ignite().mount("/api", routes![create_game]).launch();
}
