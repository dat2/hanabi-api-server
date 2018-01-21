#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket::State;
use rocket::http::{Cookie, Cookies, Method};
use rocket::response::status::NoContent;
use rocket_contrib::Json;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
use uuid::Uuid;

/* State */
#[derive(Debug)]
struct GameState {
  creator: String
}

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

  fn create_new_game(&self, creator: String) -> Uuid {
    let uuid = Uuid::new_v4();
    let state = GameState { creator: creator };
    {
      let mut w = self.games.write().unwrap();
      w.insert(uuid, state);
    }
    uuid
  }
}

/* api */
#[derive(Debug, Serialize)]
struct CreateGameResponse {
  id: Uuid,
}

#[post("/games")]
fn create_game(games_list: State<GamesState>, cookies: Cookies) -> Json<CreateGameResponse> {
  let creator = cookies.get("name")
    .map(|cookie| cookie.value())
    .unwrap_or("anonymous");
  let uuid = games_list.create_new_game(creator.to_owned());
  Json(CreateGameResponse { id: uuid })
}

#[derive(Debug, Deserialize)]
struct SetNameRequest {
  name: String
}

#[post("/name", format = "application/json", data = "<set_name>")]
fn set_name(set_name: Json<SetNameRequest>, mut cookies: Cookies) -> NoContent {
  cookies.add(Cookie::new("name", set_name.name.clone()));
  NoContent
}

fn main() {
  let client_server_origin = env::var("CLIENT_SERVER_ORIGIN").unwrap_or("http://localhost:3000".to_string());

  let (allowed_origins, _) = AllowedOrigins::some(&[&client_server_origin]);
  let fairing = rocket_cors::Cors {
    allowed_origins: allowed_origins,
    allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
    allowed_headers: AllowedHeaders::some(&["Content-Type"]),
    allow_credentials: true,
    ..Default::default()
  };


  rocket::ignite()
    .attach(fairing)
    .mount("/api", routes![set_name, create_game])
    .manage(GamesState::new())
    .launch();
}
