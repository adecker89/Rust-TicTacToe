extern crate test;

use std::fmt;
use std::io;
use std::int;
use board::{Board,BoardState, Cell};
use board::{PlayerWins,AiWins,CatsGame,InProgress};
use board::{Player,Ai,Empty};
use ai::{minimax,minimax_alpha_beta,MinimaxDelegate};
use test::Bencher;

mod board;
mod ai;

pub struct TicTacToeAiDelegate;

impl TicTacToeAiDelegate {
    fn score_cell(&self, board : &mut Board, cell : &Cell) -> uint{
        let directions = [(0,1),(1,0),(1,1),(1,-1)];
        directions.iter().fold(0, |a, &dir| a + self.score_cell_for_direction(board,cell,dir))
    }

    fn score_cell_for_direction(&self, board : &mut Board, cell : &Cell, dir : (int,int)) -> uint{
        let mut iters = [box board.iter(cell,dir) as Box<Iterator<&Cell>>,
                         box board.iter(cell,dir).rev() as Box<Iterator<&Cell>>];

        let iter = board.iter(cell,dir);
        iter.enumerate();

        let mark = cell.mark;

        //for iter in iters.move_iter() {
                // iter.nth(5);
                // let advance_iter = iter.enumerate().advance(
                //     |(idx,cell)| idx < board.get_k() && !(cell.mark == mark || cell.mark == Empty)
                // );

                // let mark_count = advance_iter.fold(0,|a,&(idx,cell)| a);
                // for cell in iter{

                //         if cell.mark == changedCell.mark || (!breakOnEmpty && cell.mark == Empty) {
                //             count+=1;
                //         } else {
                //             break;
                //         }
                //     }
                // }
        //}
        0
    }
}


impl ai::MinimaxDelegate<Board,(uint,uint)> for TicTacToeAiDelegate {
    fn possible_moves<'a> (&self, current_state : &'a mut Board, depth : uint) -> Vec<(uint,uint)> {
        current_state.cells_with_mark(Empty).iter().map(|&cell| (cell.x,cell.y)).collect()
    }

    fn do_move(&self, board : & mut Board, move : &(uint,uint), depth : uint) {
        let mark_type = if self.shouldMaximize(board,depth) { Ai } else { Player };
        let state = match move {
            &(x,y) => board.set_mark(x,y,mark_type)
        };
    }

    fn should_continue(&self, board : & mut Board, depth : uint) -> bool{
        match board.get_state() {
            PlayerWins | AiWins | CatsGame => false,
            _ => true
        }
    }

    fn undo_move(&self, board : & mut  Board, move : &(uint,uint), depth : uint) {
        match move {
            &(x,y) => board.set_mark(x,y,Empty)
        };
    }

    fn score(&self, board : & mut  Board, depth : uint) -> int {
        match board.get_state() {
            AiWins => return int::MAX - depth as int,
            PlayerWins => return int::MIN + depth as int,
            _ => ()
        }       
        let directions = [(0,1),(1,0),(1,1),(1,-1)];
        let mut score = 0;
        for cell in board.cells_with_mark(Ai).move_iter() {
            score+= directions.iter().fold(0, |sum, &dir| sum + board.count_consecutive(cell,dir,true)) as int * 100;
        }

        for cell in board.cells_with_mark(Player).move_iter() {
            score-= directions.iter().fold(0, |sum, &dir| sum + board.count_consecutive(cell,dir,true)) as int * 100;
        }        

        score
    }

    fn shouldMaximize(&self, board : & mut  Board, depth : uint) -> bool {
        depth % 2 == 0
    }

    fn max_plies(&self) -> uint {
        5
    }
}

pub enum MoveResult {
    InvalidMove,
    AiMove((uint,uint)),
    GameOver(BoardState)
}

impl fmt::Show for MoveResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &InvalidMove => write!(f.buf,"Invalid Move"),
            &AiMove((x,y)) => write!(f.buf,"Ai places O at {},{}",x,y),
            &GameOver(PlayerWins) => write!(f.buf,"Player Wins!"),
            &GameOver(AiWins) => write!(f.buf,"Ai Wins!"),
            _ => write!(f.buf,""),
        }
    }
}

pub fn make_move(board: &mut Board, x : uint, y : uint) -> MoveResult {
    if x >= board.get_num_cols() || y >= board.get_num_rows() {
        return InvalidMove;
    } else {
        match board.get_cell(x,y).mark {
            Empty => {
                let state = board.set_mark(x,y,Player);
                match state {
                    w @ PlayerWins | w @ AiWins | w @ CatsGame => return GameOver(w),
                    InProgress => ()
                }

                let ai_move = get_ai_move(board);
                let state = match ai_move {
                    (x,y) => board.set_mark(x,y,Ai)
                };
                match state {
                    w @ PlayerWins | w @ AiWins | w @ CatsGame => return GameOver(w),
                    InProgress => AiMove(ai_move)
                }
            },
            _ =>InvalidMove
        }
    }
}

fn get_ai_move(board: &Board) -> (uint, uint) {
    get_ai_move_minimax(board)
}

fn get_ai_move_simple(board: &Board) -> (uint, uint) {    
   let cells = board.cells_with_mark(Empty);
   let cell = cells.get(0);
   (cell.x,cell.y)
}

fn get_ai_move_minimax(board: &Board) -> (uint, uint) {
    let delegate = box TicTacToeAiDelegate as Box<MinimaxDelegate<Board,(uint,uint)>>;
    let mut scrap_board = board.clone();
    match minimax_alpha_beta(delegate,&mut scrap_board) {
        (move,_) => move
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

    loop {
        println!("{}",board);
        println!("Enter a move: x,y");
        let move = read_move();

        let moveResult = match move {
            Some((x,y)) => make_move(&mut board,x,y),
            None => { println!("Invalid input"); continue; }
        };
        println!("{}",moveResult);
        match moveResult {
            GameOver(_) => break,
            _ => ()
        }
    }
    println!("{}",board);
}

#[bench]
fn spar(b: &mut Bencher) {
   let mut board = Board::new(5,5,4);

    loop {
        println!("{}",board);
        println!("Enter a move: x,y");
        let (x,y) = get_ai_move(&board);

        let moveResult = make_move(&mut board,x,y);
        println!("{}",moveResult);
        match moveResult {
            GameOver(_) => break,
            _ => ()
        }
    }
    println!("{}",board);
}

