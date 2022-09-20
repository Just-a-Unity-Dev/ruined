use raylib::prelude::consts::KeyboardKey::*;
use raylib::prelude::{*};
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

    let mut player: Player = Player {x: 0, y: 0, walk_speed: 3};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_down(KEY_RIGHT) { player.x += player.walk_speed }
        if d.is_key_down(KEY_LEFT) { player.x -= player.walk_speed }
        if d.is_key_down(KEY_UP) { player.y -= player.walk_speed }
        if d.is_key_down(KEY_DOWN) { player.y += player.walk_speed }


        d.clear_background(colors::TAURI);
        d.draw_rectangle(player.x, player.y, 24, 24, Color::RED);
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
    }
}

fn input(rl: RaylibBuilder) -> PlayerAction {
    PlayerAction::DidntMove
}
