extern crate test;
extern crate sync;

use std::io;
use board::{Board};
use board::{InProgress};
use board::{Player,Ai,Empty};
use ai::{Ai};
use ai::{Simple,Minimax,AlphaBeta};

mod board;
mod ai;
mod minimax;

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
    let mut board = Board::new(3,3,3);
    // let num_cells = board.get_num_rows() * board.get_num_cols();

    //decreasing function that gives 9 plies for a board with 9 cells and a minimum of 3 plies
    // let plies = 3 * (num_cells + 18) / num_cells;
    // println!("{}",plies);
    let ai = Ai::new(AlphaBeta, 9);

    loop {
        println!("{}",board);
        println!("Enter a move: x,y");

        let move = match read_move() {
            None => { 
                println!("Invalid input"); 
                continue; 
            }
            Some(move) =>{
                if !board.check_valid_move(move) {
                    println!("Invalid move");
                    continue;
                }
                move
            }
        };        
        board.set_mark(move,Player);
        match board.get_state() {
            InProgress => (),
            _ => break
        }

        let ai_move = ai.get_move(&board);
        board.set_mark(ai_move,Ai);
        match board.get_state() {
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

