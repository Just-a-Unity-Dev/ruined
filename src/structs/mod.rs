pub struct Player {
    pub x: i32,
    pub y: i32,
}

pub enum PlayerAction {
    TookMove,
    DidntMove
}