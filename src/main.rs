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

    let mut player: Player = Player {x: 0, y: 0};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_down(KEY_RIGHT) { player.x += 2} ;
        if d.is_key_down(KEY_LEFT) { player.x -= 2} ;
        if d.is_key_down(KEY_UP) { player.y -= 2} ;
        if d.is_key_down(KEY_DOWN) { player.y += 2} ;


        d.clear_background(colors::TAURI);
        d.draw_rectangle(player.x, player.y, 24, 24, Color::RED);
        d.draw_text("Hello, world!", 12, 12, 20, Color::WHITE);
    }
}

fn input(rl: RaylibBuilder) -> PlayerAction {
    PlayerAction::DidntMove
}
