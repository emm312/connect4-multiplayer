use std::process::exit;

use ansi_escapes::{CursorUp, EraseLine};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpListener;

use crate::board::{State, Board};

use crate::input;

pub async fn start() {
    let mut board = Board::new();
    let mut move_up_num = 0;

    let ip = "127.0.0.1:54389";

    let listener = TcpListener::bind(ip).await.unwrap();
    println!("Waiting for opponent to join...");
    let (mut socket, _) = listener.accept().await.unwrap();

    loop {
        println!("{}", CursorUp(move_up_num));
        println!("{board}");
        print!("{}", EraseLine);
        if let Some(winner) = board.wincheck() {
            println!("{} wins!", winner);
            exit(0);
        }
        print!("Your move: ");
        let mov = input();

        loop {
            match mov.parse::<usize>() {
                Ok(n) => match board.place_obj(State::X, n.clamp(1, 7) - 1) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("{e}");
                        move_up_num += 1;
                        continue;
                    }
                },
                Err(_) => {
                    println!("Enter a valid number.");
                    continue;
                }
            }
            break;
        }

        socket.write_u8(mov.parse::<u8>().unwrap().clamp(1, 7)).await.unwrap();
        print!("{}", CursorUp(9));
        println!("{board}");
        print!("{}", EraseLine);
        let mov = socket.read_u8().await.unwrap();
        board.place_obj(State::O, mov as usize-1).unwrap();

        move_up_num = 9;
    }
}