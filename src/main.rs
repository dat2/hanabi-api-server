#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use rocket::{Request, State, Outcome};
use rocket::http::{Cookie, Cookies, Method};
use rocket::request::{self, FromRequest};
use rocket::response::status::NoContent;
use rocket_contrib::{Json, UUID};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
use uuid::Uuid;

/* request guards */
#[derive(Clone, Debug, Serialize)]
struct Player {
  name: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Player {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Player, ()> {
    let cookies = request.guard::<Cookies>()?;

    let name = cookies
      .get("name")
      .map(|cookie| cookie.value())
      .unwrap_or("anonymous")
      .to_owned();

    Outcome::Success(Player {
      name: name
    })
  }
}

/* State */
#[derive(Clone, Debug, Serialize)]
struct Game {
  id: Uuid,
  creator: String,
  players: Vec<Player>,
  name: String,
  max_players: usize,
  password: Option<String>
}

impl Game {
  fn add_player(&mut self, player: Player) {
    self.players.push(player);
  }
}

#[derive(Debug)]
struct GamesState {
  games: RwLock<HashMap<Uuid, Game>>,
}

impl GamesState {
  fn new() -> GamesState {
    GamesState {
      games: RwLock::new(HashMap::new()),
    }
  }

  fn create_new_game(&self, creator: String, name: String, max_players: usize, password: Option<String>) -> Game {
    let uuid = Uuid::new_v4();
    let game = Game {
      id: uuid.clone(),
      creator: creator,
      players: Vec::new(),
      name: name,
      max_players: max_players,
      password: password
    };
    {
      let mut w = self.games.write().unwrap();
      w.insert(uuid, game.clone());
    }
    game
  }

  fn add_player_to_game(&self, game_id: &Uuid, player: Player) {
    {
      let mut w = self.games.write().unwrap();
      if let Some(game) = w.get_mut(game_id) {
        game.add_player(player);
      }
    }
  }
}

/* api */
#[derive(Debug, Deserialize)]
struct SetNameRequest {
  name: String,
}

#[post("/name", format = "application/json", data = "<set_name>")]
fn set_name(set_name: Json<SetNameRequest>, mut cookies: Cookies) -> NoContent {
  let cookie = Cookie::build("name", set_name.name.clone())
    .path("/")
    .finish();
  cookies.add(cookie);
  NoContent
}

#[derive(Debug, Deserialize)]
struct CreateGameRequest {
  name: String,
  players: usize,
  password: Option<String>
}

#[derive(Debug, Serialize)]
struct CreateGameResponse {
  creator: Player,
  game: Game
}

#[post("/games", format = "application/json", data = "<game_options>")]
fn create_game(games_list: State<GamesState>, player: Player, game_options: Json<CreateGameRequest>) -> Json<CreateGameResponse> {
  let game = games_list.create_new_game(
    player.name.clone(),
    game_options.name.clone(),
    game_options.players,
    game_options.password.clone()
  );
  Json(CreateGameResponse {
    creator: player,
    game: game,
  })
}

#[post("/games/<id>/join")]
fn join_game(id: UUID, games_list: State<GamesState>, player: Player) -> NoContent {
  games_list.add_player_to_game(&*id, player);
  NoContent
}

fn main() {
  let client_server_origin =
    env::var("CLIENT_SERVER_ORIGIN").unwrap_or("http://localhost:3000".to_string());

  let (allowed_origins, _) = AllowedOrigins::some(&[&client_server_origin]);
  let fairing = rocket_cors::Cors {
    allowed_origins: allowed_origins,
    allowed_methods: vec![Method::Get, Method::Post, Method::Options]
      .into_iter()
      .map(From::from)
      .collect(),
    allowed_headers: AllowedHeaders::some(&["Content-Type"]),
    allow_credentials: true,
    ..Default::default()
  };

  rocket::ignite()
    .attach(fairing)
    .mount("/api", routes![set_name, create_game, join_game])
    .manage(GamesState::new())
    .launch();
}
