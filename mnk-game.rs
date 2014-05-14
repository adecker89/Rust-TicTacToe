
#![crate_type = "staticlib"]

extern crate sync;
extern crate libc;

use libc::types::os::arch::c95::c_uint;
use board::{Board,BoardState};
use board::{PlayerWins,AiWins,CatsGame,InProgress};
use main::{AiMove,InvalidMove,GameOver};
use main::TicTacToeAiDelegate;
use ai::{minimax,MinimaxDelegate};

mod main;
mod board;
mod ai;


static INVALID_MOVE : c_uint = 0;
static PLAYER_WINS : c_uint = 1;


#[no_mangle]
pub extern "C" fn init_board(m : c_uint, n : c_uint, k : c_uint) -> Box<Board> { 
    let board = box Board::new(m as uint,n as uint,k as uint);
    board
}

#[no_mangle]
pub extern "C" fn make_move(board: &mut Board, x : c_uint, y : c_uint, xout : *mut c_uint, yout : *mut c_uint) -> c_uint {
    let my_move = main::make_move(board, x as uint,y as uint);
    match my_move {
        AiMove((my_x, my_y)) => unsafe {
            *yout = my_y as c_uint; 
            *xout = my_x as c_uint; 
            return 1;
        },
        InvalidMove => return 0,
        GameOver(state) => state as c_uint            
    }
}

#[no_mangle]
pub extern "C" fn print_board(board: & Board) {
    println!("{}",board);
}
