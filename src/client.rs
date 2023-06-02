use std::process::exit;

use ansi_escapes::{CursorUp, EraseLine};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpSocket;

use crate::board::{State, Board};

use crate::input;

pub async fn start() {
    let mut board = Board::new();
    let mut move_up_num = 0;

    print!("Input the ip: ");
    let ip = format!("{}:54389", input());

    let listener = TcpSocket::new_v4().unwrap();
    let mut socket = listener.connect(ip.parse().unwrap()).await.unwrap();
    println!("{}", board);
    print!("{}", CursorUp(9));

    loop {
        println!("{}", CursorUp(move_up_num));
        let mov = socket.read_u8().await.unwrap();
        board.place_obj(State::X, mov as usize-1).unwrap();
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
                Ok(n) => match board.place_obj(State::O, n.clamp(1, 7) - 1) {
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


        print!("{}", CursorUp(9));
        println!("{board}");


        socket.write_u8(mov.parse::<u8>().unwrap().clamp(1, 7)).await.unwrap();


        move_up_num = 9;
    }
}