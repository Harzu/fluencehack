use crate::error_type::AppResult;
use crate::game::{Game, GameMove, Tile};
use crate::player::Player;
use crate::request_response::Response;

use crate::settings::{GAMES_MAX_COUNT, PLAYERS_MAX_COUNT, USER_NAME_MAX_LEN};
use arraydeque::{ArrayDeque, Wrapping};
use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, ops::AddAssign, rc::Rc, rc::Weak};

pub struct GameStatistics {
    // overall players count that has been registered
    pub players_created: u64,
    // overall players count that has been created
    pub games_created: u64,
    // overall move count that has been made
    pub moves_count: u64,
}

pub struct TGame {
    room_id: String,
    players: Vec<(String, [usize; 10])>
}

pub struct GameManager {
    players: ArrayDeque<[Rc<RefCell<Player>>; PLAYERS_MAX_COUNT], Wrapping>,
    games: ArrayDeque<[Rc<RefCell<TGame>>; GAMES_MAX_COUNT], Wrapping>,
    // TODO: String key should be replaced with Cow<'a, str>. After that signatures of all public
    // functions also should be changed similar to https://jwilm.io/blog/from-str-to-cow/.
    players_by_name: HashMap<String, Weak<RefCell<Player>>>,
    game_statistics: RefCell<GameStatistics>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            games: ArrayDeque::new(),
            players: ArrayDeque::new(),
            players_by_name: HashMap::new(),
            game_statistics: RefCell::new(GameStatistics {
                players_created: 0,
                games_created: 0,
                moves_count: 0,
            }),
        }
    }

    pub fn create_game(&mut self, player_name: String, room_id: String, army: [usize; 10]) -> AppResult<Value> {
        let game = Rc::new(RefCell::new(TGame {
            room_id,
            players: vec![(player_name, army)]
        }));
        
        self.games.push_back(game);
        let response = Response::CreateBattle { state: "done".to_string() };
        serde_json::to_value(response).map_err(Into::into)
    }

    pub fn get_rooms(&self, count: usize) -> AppResult<Value> {
        let mut rooms = vec![];
        for item in &self.games {
            let room = &item.borrow().room_id;
            rooms.push(room.clone());
        }

        let response = Response::GetRooms { rooms };
        serde_json::to_value(response).map_err(Into::into)
    }

    pub fn connect(&mut self, player_name: String, room_id: String, army: [usize; 10]) -> AppResult<Value> {
        for item in &self.games {
            if &item.borrow().room_id == &room_id {
                let players = &mut item.borrow_mut().players;
                players.push((player_name.clone(), army));
            }
        }

        let response = Response::Connect { state: "done".to_string() };
        serde_json::to_value(response).map_err(Into::into)
    }

    pub fn get_users(&self, room_id: String) -> AppResult<Value> {
        let mut pl = vec![];
        for item in &self.games {
            if &item.borrow().room_id == &room_id {
                pl = item.borrow_mut().players.clone();
            }
        }

        let response = Response::GetUsers { users: pl };
        serde_json::to_value(response).map_err(Into::into)
    }
}
