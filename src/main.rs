use raylib::prelude::consts::KeyboardKey::*;
use raylib::prelude::{*};

use rand::Rng;

use structs::*;
use consts::*;

pub mod structs;
pub mod consts;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(screen::WIDTH, screen::HEIGHT)
        .title("Ruined")
        .build();
    
    rl.set_target_fps(60);

    let x: i32 = rand::thread_rng().gen_range(0..screen::WIDTH);
    let y: i32 = rand::thread_rng().gen_range(0..screen::HEIGHT);
    let mut player: Player = Player {x: x, y: y, walk_speed: 3, health: 5, max_health: 5, materials: 0};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_down(KEY_RIGHT) { player.x += player.walk_speed }
        if d.is_key_down(KEY_LEFT) { player.x -= player.walk_speed }
        if d.is_key_down(KEY_UP) { player.y -= player.walk_speed }
        if d.is_key_down(KEY_DOWN) { player.y += player.walk_speed }


        d.clear_background(colors::TAURI);
        d.draw_rectangle(player.x, player.y, 24, 24, Color::RED);
    }
}

fn input(rl: RaylibBuilder) -> PlayerAction {
    PlayerAction::DidntMove
}
