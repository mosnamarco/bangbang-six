use crossbeam::channel::bounded;
use raylib::prelude::*;
use std::sync::mpsc;

mod network_logic;
use network_logic::{handle_client, handle_server_connect};

fn main() {
    println!("Choose mode\n1. Server\n2. Client");
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Error reading line");

    let (game_tx, game_rx) = mpsc::channel();
    let (server_tx, server_rx) = bounded(0);

    match choice.trim() {
        "1" => {
            println!("Server stuff");
            std::thread::spawn(move || {
                handle_client(game_tx.clone(), server_rx.clone());
            });
        }
        "2" => {
            println!("Client stuff");
            handle_server_connect(game_tx.clone(), server_rx.clone());
        }
        _ => {
            println!("Invalid input, defaulting to server...");
            std::thread::spawn(move || {
                handle_client(game_tx.clone(), server_rx.clone());
            });
        }
    }

    let (mut rl, thread) = raylib::init().size(640, 480).title("bangbang_six").build();
    rl.set_target_fps(60);

    let mut message = String::new();
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let num: i32 = get_random_value(0, 6);
            if num == 6 {
                server_tx.send("BANG!".to_string()).unwrap();
            } else {
                server_tx.send(num.to_string()).unwrap();
            }
            println!("space presed");
        }

        let mut d = rl.begin_drawing(&thread);

        if let Ok(recieved_msg) = game_rx.try_recv() {
            message = recieved_msg;
        }

        if message == "BANG!" {
            d.draw_text("AGAYyy!!!!", 100, 200, 40, Color::RED);
        } else {
            d.draw_text(&message, 100, 100, 20, Color::BLACK);
        }

        d.clear_background(Color::WHITE);
        d.draw_fps(10, 10);
    }
}
