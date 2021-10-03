use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use chess_engine::chess_game::Game;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

pub(crate) struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    tx: Sender<(String, SocketAddr)>,
    rx: Receiver<(String, SocketAddr)>,
}

pub(crate) fn start_server() -> Server {
    let listener = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    listener
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<(String, SocketAddr)>();

    return Server {
        listener,
        clients,
        tx,
        rx,
    };
}

pub(crate) fn server_loop(server: &mut Server, game: &mut Game) -> Result<(),String> {
    if let Ok((mut socket, addr)) = server.listener.accept() {
        println!("Client {} connected", addr);

        let tx = server.tx.clone();
        server
            .clients
            .push(socket.try_clone().expect("failed to clone client"));

        thread::spawn(move || loop {
            let mut buff = vec![0; MSG_SIZE];

            match socket.read_exact(&mut buff) {
                Ok(_) => {
                    // 0x3B is unicode for ';'
                    let msg = buff
                        .into_iter()
                        .take_while(|&x| (x != 0 && x != 0x3B))
                        .collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                    println!("{}: {:?}", addr, msg);
                    tx.send((msg, addr)).expect("failed to send msg to rx");
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("closing connection with: {}", addr);
                    break;
                }
            }

            thread::sleep(std::time::Duration::from_millis(100)); // every connection gets a thread it looks
        });
    }

    loop {
        // do chess logic here
        if let Ok((msg, addr)) = server.rx.try_recv() {
            let mut send_msg = msg.clone();

            let mut send_to_all = true;

            let split: Vec<String> = msg.split(":").map(|s| s.to_string()).collect();
            if split.len() != 2 {
                return Ok(());
            }

            let action = &split[0];
            let input = &split[1];

            match &action[..] {
                "move" => {
                    let result = game.algebraic_notation_move(input.to_string());
                    if result.is_ok() {
                        println!("Move Succesfull!");
                    } else {
                        println!("Move Failed!");
                        let error_message = result.err().unwrap();
                        println!("{}", error_message);
                    }
                    send_msg = "moved".to_string();
                }
                "turn" => {
                    send_to_all = false;
                    send_msg = (if game.turn == chess_engine::chess_game::ChessPieceColor::White {
                        "white"
                    } else {
                        "black"
                    })
                    .to_string()
                }
                _ => {
                    println!("Normal msg");
                }
            }

            let mut buff = send_msg.into_bytes();
            buff.resize(MSG_SIZE, 0);

            if send_to_all {
                for client in &mut server.clients {
                    let _write_error = client.write_all(&buff);
                }
            } else {
                for client in &mut server.clients {
                    let client_addr = client.peer_addr();
                    if client_addr.is_ok() && client_addr.unwrap() == addr {
                        let _write_error = client.write_all(&buff);
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}
