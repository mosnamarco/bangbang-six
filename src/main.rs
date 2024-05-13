use crossbeam::channel::bounded;
use raylib::prelude::*;
use std::{io, sync::mpsc};

mod network_logic;
use network_logic::{handle_client, handle_server_connect};

const W_WIDTH: i32 = 800;
const W_HEIGHT: i32 = 500;

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
            handle_client(game_tx.clone(), server_rx.clone());
        }
        "2" => {
            println!("Client stuff");
            let mut ip = String::new();

            println!("Enter ip <192.x.x.x>: ");
            io::stdin().read_line(&mut ip).expect("failed to read ip");

            handle_server_connect(game_tx.clone(), server_rx.clone(), ip.trim().to_string());
        }
        _ => {
            println!("Invalid input, defaulting to server...");
            handle_client(game_tx.clone(), server_rx.clone());
        }
    }

    let (mut rl, thread) = raylib::init()
        .size(W_WIDTH, W_HEIGHT)
        .title("bangbang_six")
        .build();
    rl.set_target_fps(60);

    let mut audio_handler = raylib::audio::RaylibAudio::init_audio_device();
    let gun_click: Sound = Sound::load_sound("resources/gun_click.mp3").unwrap();
    let gun_pop: Sound = Sound::load_sound("resources/gun_pop.mp3").unwrap();
    audio_handler.set_sound_volume(&gun_click, 0.4);

    let mut message = String::new();
    let mut winner = false;
    let mut game_start: bool = false;

    while !rl.window_should_close() {
        if let Ok(recieved_msg) = game_rx.try_recv() {
            message = recieved_msg;
        }

        if game_start == false {
            server_tx.send("START".to_string()).unwrap();
            game_start = message.trim() == "START";
            message.clear();
        }

        // NOTE: Update
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            && message.trim() != "BANG!"
            && winner == false
            && game_start == true
        {
            let num: i32 = get_random_value(0, 6);
            if num == 6 {
                server_tx.send("BANG!".to_string()).unwrap();
                winner = true;
                audio_handler.play_sound(&gun_pop);
            } else {
                server_tx.send(num.to_string()).unwrap();
                audio_handler.play_sound(&gun_click);
            }
            println!("Bullet sent!");
        }

        // NOTE: Draw
        let mut d = rl.begin_drawing(&thread);

        match winner {
            true => {
                d.draw_text("YOU WIN!!!", 10, 70, 40, Color::GREEN);
            }
            false => match message.as_str() {
                "BANG!" => {
                    d.draw_text("YOU LOSE...", 10, 70, 40, Color::RED);
                }
                _ => {
                    d.draw_text(
                        &format!("His number: {}", &message.trim()).to_string(),
                        10,
                        70,
                        30,
                        Color::BLUE,
                    );
                }
            },
        }

        match choice.trim() {
            "1" => {
                d.draw_text("You are server", 10, 30, 20, Color::BLACK);
            }
            "2" => {
                d.draw_text("You are client", 10, 30, 20, Color::BLACK);
            }
            _ => {}
        }
        d.clear_background(Color::WHITE);
        d.draw_fps(10, 10);
    }
}
