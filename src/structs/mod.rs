pub struct Player {
    pub x: i32,
    pub y: i32,
    pub walk_speed: i32,
    pub health: i32,
    pub max_health: i32,
    pub materials: i32,
}

pub enum PlayerAction {
    TookMove,
    DidntMove
}