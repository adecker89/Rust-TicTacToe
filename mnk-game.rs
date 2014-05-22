
#![crate_type = "staticlib"]

extern crate libc;
extern crate test;

use libc::types::os::arch::c95::c_uint;
use board::{Board,BoardState};
use board::{InProgress,PlayerWins,AiWins,CatsGame};
use board::{Player,Ai};
use ai::{Ai};
use ai::{Simple,Minimax,AlphaBeta};

mod board;
mod ai;
mod minimax;

static INVALID_MOVE : c_uint = 0;
static PLAYER_WINS : c_uint = 1;
static AI_WINS : c_uint = 2;
static CATS_GAME : c_uint = 3;
static IN_PROGRESS : c_uint = 4;

fn const_for_boardstate(state : BoardState) -> c_uint {
     match state {
        PlayerWins => PLAYER_WINS,
        AiWins => AI_WINS,
        CatsGame => CATS_GAME,
        InProgress => IN_PROGRESS,
    }
}

#[no_mangle]
pub extern "C" fn init_board(m : c_uint, n : c_uint, k : c_uint) -> Box<Board> { 
    let board = box Board::new(m as uint,n as uint,k as uint);
    board
}

#[no_mangle]
pub extern "C" fn destroy_board(board: Box<Board>) {
    //returns ownership of board. memory will be freed
}

#[no_mangle]
pub extern "C" fn make_move(board: &mut Board, x : c_uint, y : c_uint, xout : *mut c_uint, yout : *mut c_uint) -> c_uint {
    let move = (x as uint, y as uint);
    if !board.check_valid_move(move) {
        return INVALID_MOVE;
    }

    board.set_mark(move,Player);

    let const_state = const_for_boardstate(board.get_state());
    if const_state != IN_PROGRESS {
        return const_state;
    }

    let ai_move = Ai::new(AlphaBeta, 9).get_move(board);

    
    match ai_move {(x,y) => unsafe {
        *xout = x as c_uint;
        *yout = y as c_uint;
    }};
    

    board.set_mark(ai_move,Ai);
    return const_for_boardstate(board.get_state());
}

#[no_mangle]
pub extern "C" fn print_board(board: & Board) {
    println!("{}",board);
}


