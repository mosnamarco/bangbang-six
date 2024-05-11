use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use local_ip_address::linux::local_ip;
use raylib::prelude::*;

fn handle_client() {
    let ip = local_ip();
    let addr = format!("{}:{}", ip.unwrap(), 6969);
    let listener = TcpListener::bind(addr).expect("failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_arc = Arc::new(stream);

                let w_stream = Arc::clone(&stream_arc);
                std::thread::spawn(move || {
                    let mut w = BufWriter::new(w_stream.as_ref());

                    // TODO: make this available in main thread
                    loop {
                        let mut input = String::new();
                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read from input");

                        let message = format!("Hello from server: {}\n", input.trim());
                        w.write_all(message.as_bytes())
                            .expect("Failed to write to client");
                        w.flush().expect("Failed to flush buffer");
                    }
                });

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
                            println!("{}", message.trim());
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

fn handle_server_connect() {
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
                println!("{}", message.trim());
            }
        }
    });

    let w_socket = Arc::clone(&socket);
    std::thread::spawn(move || {
        let mut w = BufWriter::new(w_socket.as_ref());

        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from input");

            let message = format!("Hello from client: {}\n", input.trim());
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

    match choice.trim() {
        "1" => {
            println!("Server stuff");
            handle_client();
        }
        "2" => {
            println!("Client stuff");
            handle_server_connect();
        }
        _ => {
            println!("Invalid input, defaulting to server...");
            handle_client();
        }
    }

    let (mut rl, thread) = raylib::init().size(640, 480).title("bangbang_six").build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_fps(10, 10);
    }
}
