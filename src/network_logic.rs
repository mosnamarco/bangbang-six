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

    match listener.accept() {
        Ok((socket, addr)) => {
            println!("Connected to: {}", addr);

            let socket = Arc::new(socket);

            let reader_socket = Arc::clone(&socket);
            std::thread::spawn(move || loop {
                let mut reader = BufReader::new(reader_socket.as_ref());

                let mut message = String::new();
                let _ = reader.read_line(&mut message);

                game_tx.send(message.trim().to_string()).unwrap();
                println!("{}", message.trim());
            });

            let writer_socket = Arc::clone(&socket);
            std::thread::spawn(move || loop {
                let mut writer = BufWriter::new(writer_socket.as_ref());

                let mut message = String::new();
                if let Ok(recieved_msg) = server_rx.try_recv() {
                    message = format!("{}\n", recieved_msg.trim());
                }
                let _ = writer.write_all(message.as_bytes());
            });
        }
        Err(_) => {
            println!("Failed to connect to client");
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
