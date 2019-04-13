/*
 * Copyright 2018 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    CreateBattle {
        player_name: String,
        room_id: String,
        army: [usize; 10]
    },
    Connect {
        room_id: String,
        player_name: String,
        army: [usize; 10]
    },
    GetRooms { count: usize },
    GetUsers { room_id: String }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    CreateBattle { state: String },
    Connect { state: String },
    GetRooms { rooms: Vec<String> },
    GetUsers { users: Vec<(String, [usize; 10])> },
    Error { error: String },
}
