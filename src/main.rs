#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate uuid;
extern crate ws;

use rocket::State;
use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::RwLock;
use std::thread;
use uuid::Uuid;
use ws::listen;

#[derive(Debug)]
struct GameState;

#[derive(Debug)]
struct GamesState {
  games: RwLock<HashMap<Uuid, GameState>>
}

impl GamesState {
  fn new() -> GamesState {
    GamesState { games: RwLock::new(HashMap::new()) }
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
  id: Uuid
}

#[post("/games")]
fn create_game(games_list: State<GamesState>) -> Json<CreateGameResponse> {
  let uuid = games_list.create_new_game();
  Json(CreateGameResponse { id: uuid })
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

  rocket::ignite()
    .mount("/api", routes![create_game])
    .manage(GamesState::new())
    .launch();
}
