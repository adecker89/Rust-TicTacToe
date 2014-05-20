extern crate test;
extern crate sync;

use std::fmt;
use std::io;
use board::{Board, BoardState, Cell, Mark};
use board::{PlayerWins,AiWins,CatsGame,InProgress};
use board::{Player,Ai,Empty};
use ai::{Ai};
use ai::{Simple,Minimax,AlphaBeta};

use test::Bencher;

mod board;
mod ai;
mod minimax;

pub fn check_valid_move(board: &Board, move : (uint,uint)) -> bool {
    match move {
        (x,y) => {
            if x >= board.get_num_cols() || y >= board.get_num_rows() {
                return false;
            }
            board.get_cell(x,y).mark == Empty
        }
    }
}

fn read_move() -> Option<(uint,uint)> {
    let n = io::stdin().read_line().unwrap();
    let mut parts : Vec<uint> = n.as_slice().split(',').filter_map(|s| from_str(s.trim())).collect();

    if parts.len() == 2 {
        let x = parts.swap_remove(0).unwrap();
        let y = parts.swap_remove(0).unwrap();
        Some( (x,y) )
    } else {
        None
    }
}

fn main() {
    let mut board = Board::new(5,5,4);
    let ai = Ai::new(AlphaBeta,6);

    loop {
        println!("{}",board);
        println!("Enter a move: x,y");

        let move = match read_move() {
            None => { 
                println!("Invalid input"); 
                continue; 
            }
            Some(move) =>{
                if !check_valid_move(&board,move) {
                    println!("Invalid move");
                    continue;
                }
                move
            }
        };        

        match board.set_mark(move,Player) {
            InProgress => (),
            _ => break
        }

        let ai_move = ai.get_move(&board);
        match board.set_mark(ai_move,Ai) {
            InProgress => (),
            _ => break
        }
        match ai_move {(x,y) => println!("Ai places O at {},{}",x,y)};
    }
    println!("{}",board.get_state());
    println!("{}",board);
}

#[bench]
fn first_move_mm(b: &mut Bencher) {
    let ai = Ai::new(Minimax,3);
    b.iter(|| {
       let board = Board::new(5,5,4);
       ai.get_move(&board)
    });
}

#[bench]
fn first_move_ab(b: &mut Bencher) {
    let ai = Ai::new(AlphaBeta,3);
    b.iter(|| {
       let board = Board::new(5,5,4);
       ai.get_move(&board)
    });
}

