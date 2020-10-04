use crate::player::Player;

#[derive(Debug)]
pub struct World {
    starting_room: Room,
}

impl World {
    pub fn enter_world(&mut self, mut player: Player) {
        self.starting_room.players.push(player);
    }
}

#[derive(Debug)]
struct Room {
    X: i32,
    Y: i32,
    Z: i32,
    name: String,
    desc: String,
    players: Vec<Player>,
}

pub fn test_new_world() -> World {
    World {
        starting_room: Room {
            X: 1,
            Y: 2,
            Z: 3,
            name: String::from("Test Room"),
            desc: String::from("A Noble Description"),
            players: Vec::new(),
        }
    }
}