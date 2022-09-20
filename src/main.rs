use raylib::prelude::consts::KeyboardKey::*;
use raylib::prelude::{*};

use std::io::*;
use std::process::*;

use rand::Rng;

use structs::*;
use consts::*;

pub mod structs;
pub mod consts;

fn attempt_connect() {
    // TODO: add proper connection
    println!("Insert connection address.");
    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("Now connecting to {}...", input);
        },
        Err(_error) => {
            println!("There was an unexpected error parsing your input.");
            menu();
        }
    }
}

fn menu() {
    // TODO: add an actual menu and optionally GET US A UI
    println!("RUINED | A 2D topdown online multiplayer game");
    println!("-------");
    attempt_connect();
}

fn main() {
    // start the main menu terminal launcher thing to pause the game temporarily until connection is established
    menu();

    // initalize raylib
    let (mut rl, thread) = raylib::init()
        .size(screen::WIDTH, screen::HEIGHT)
        .title("Ruined")
        .build();
    
    rl.set_target_fps(60);

    // initalize client player
    let x: i32 = rand::thread_rng().gen_range(0..screen::WIDTH);
    let y: i32 = rand::thread_rng().gen_range(0..screen::HEIGHT);
    let mut player: Player = Player {x: x, y: y, walk_speed: 3, health: 5, max_health: 5, materials: 0};

    // game loop (what)
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // yucky input
        if d.is_key_down(KEY_RIGHT) { player.x += player.walk_speed }
        if d.is_key_down(KEY_LEFT) { player.x -= player.walk_speed }
        if d.is_key_down(KEY_UP) { player.y -= player.walk_speed }
        if d.is_key_down(KEY_DOWN) { player.y += player.walk_speed }

        // render the game
        d.clear_background(colors::TAURI);
        d.draw_rectangle(player.x, player.y, 24, 24, Color::RED);
    }

    // player exited the game
    println!("Thank you for playing RUINED");
}

fn input(rl: RaylibBuilder) -> PlayerAction {
    PlayerAction::DidntMove
}
