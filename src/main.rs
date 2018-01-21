#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket::{Request, Response, State};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket_contrib::Json;
use std::collections::HashMap;
use std::env;
use std::io::Cursor;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
struct GameState;

#[derive(Debug)]
struct GamesState {
  games: RwLock<HashMap<Uuid, GameState>>,
}

impl GamesState {
  fn new() -> GamesState {
    GamesState {
      games: RwLock::new(HashMap::new()),
    }
  }

  fn create_new_game(&self) -> Uuid {
    let uuid = Uuid::new_v4();
    let state = GameState;
    {
      let mut w = self.games.write().unwrap();
      w.insert(uuid, state);
    }
    uuid
  }
}

#[derive(Debug, Serialize)]
struct CreateGameResponse {
  id: Uuid,
}

#[post("/games")]
fn create_game(games_list: State<GamesState>) -> Json<CreateGameResponse> {
  let uuid = games_list.create_new_game();
  Json(CreateGameResponse { id: uuid })
}

struct Cors {
  origin: String,
}

impl Cors {
  fn new(origin: String) -> Cors {
    Cors { origin: origin }
  }
}

impl Fairing for Cors {
  fn info(&self) -> Info {
    Info {
      name: "Add Cors headers to requests",
      kind: Kind::Response,
    }
  }

  fn on_response(&self, request: &Request, response: &mut Response) {
    if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
      response.set_header(Header::new(
        "Access-Control-Allow-Origin",
        self.origin.clone(),
      ));
      response.set_header(Header::new(
        "Access-Control-Allow-Methods",
        "POST, GET, OPTIONS",
      ));
      response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
      response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    if request.method() == Method::Options {
      response.set_header(ContentType::Plain);
      response.set_sized_body(Cursor::new(""));
    }
  }
}

fn main() {
  let cors_origin = env::var("CLIENT_SERVER_ORIGIN").unwrap_or("http://localhost:3000".to_string());

  rocket::ignite()
    .attach(Cors::new(cors_origin))
    .mount("/api", routes![create_game])
    .manage(GamesState::new())
    .launch();
}
