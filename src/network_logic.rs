use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc},
};

use crossbeam::channel::Receiver;
use local_ip_address::linux::local_ip;

pub fn handle_client(game_tx: mpsc::Sender<String>, server_rx: Receiver<String>) {
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

pub fn handle_server_connect(
    game_tx: mpsc::Sender<String>,
    server_rx: Receiver<String>,
    ip: String,
) {
    // TODO: change this to be able to be defined by user
    let addr = format!("{}:{}", ip, 6969);
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
