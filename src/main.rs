use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc},
};

use crossbeam::channel::{bounded, Receiver};
use local_ip_address::linux::local_ip;
use raylib::prelude::*;

fn handle_client(game_tx: mpsc::Sender<String>, server_rx: Receiver<String>) {
    let ip = local_ip();
    let addr = format!("{}:{}", ip.unwrap(), 6969);
    let listener = TcpListener::bind(addr).expect("failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_arc = Arc::new(stream);

                let w_stream = Arc::clone(&stream_arc);
                let server_rx = server_rx.clone();
                std::thread::spawn(move || {
                    let mut w = BufWriter::new(w_stream.as_ref());

                    // TODO: make this available in main thread
                    loop {
                        let mut message = String::new();
                        if let Ok(recieved_msg) = server_rx.try_recv() {
                            message = format!("{}\n", recieved_msg.trim());
                        }
                        w.write_all(message.as_bytes())
                            .expect("Failed to write to client");
                        w.flush().expect("Failed to flush buffer");
                    }
                });

                let game_tx = game_tx.clone();
                let r_stream = Arc::clone(&stream_arc);
                std::thread::spawn(move || {
                    let mut r = BufReader::new(r_stream.as_ref());
                    // TODO: make this available in main thread
                    loop {
                        let mut message = String::new();
                        if let Ok(bytes_read) = r.read_line(&mut message) {
                            if bytes_read == 0 {
                                println!("Client disconnected");
                                break;
                            }

                            if message.trim() == "pisot" {
                                println!("Pisot mo man!");
                            }

                            game_tx.send(message.trim().to_string()).unwrap();
                        }
                    }
                });
            }
            Err(_) => {
                println!("client disconnected");
            }
        }
    }
}

fn handle_server_connect(game_tx: mpsc::Sender<String>, server_rx: Receiver<String>) {
    // TODO: change this to be able to be defined by user
    let ip = local_ip();
    let addr = format!("{}:{}", ip.unwrap(), 6969);
    let socket = TcpStream::connect(addr).expect("failed to bind to address");

    let socket = Arc::new(socket);

    let r_socket = Arc::clone(&socket);
    std::thread::spawn(move || {
        let mut r = BufReader::new(r_socket.as_ref());
        loop {
            let mut message = String::new();
            if let Ok(bytes_read) = r.read_line(&mut message) {
                if bytes_read == 0 {
                    println!("Server disconnected");
                    break;
                }
                game_tx.send(message.trim().to_string()).unwrap();
            }
        }
    });

    let w_socket = Arc::clone(&socket);
    std::thread::spawn(move || {
        let mut w = BufWriter::new(w_socket.as_ref());

        loop {
            let mut message = String::new();
            if let Ok(recieved_msg) = server_rx.try_recv() {
                message = format!("{}\n", recieved_msg.trim());
            }
            w.write_all(message.as_bytes())
                .expect("Failed to write to server");
            w.flush().expect("Failed to flush buffer");
        }
    });
}

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
