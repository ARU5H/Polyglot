use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use std::io::{self, Write, BufRead};

#[derive(Serialize, Deserialize)]
struct GameState{
    player_x: i32,
    player_y: i32,
}

fn main(){
    let stdin = io::stdin();
    let mut game_state = GameState{player_x: 0, player_y: 0};
    loop{
        let json = serde_json::to_string(&game_state).unwrap();
        println!("{}", json);
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
    }
}