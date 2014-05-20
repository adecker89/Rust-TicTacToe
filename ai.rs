use std::int;
use std::cmp;
use std::fmt;
use board::{Board, BoardState, Cell, Mark};
use board::{PlayerWins,AiWins,CatsGame,InProgress};
use board::{Player,Ai,Empty};
use minimax::{MinimaxDelegate,minimax,minimax_alpha_beta};

#[deriving(Clone)]
pub enum Mode {
    Simple,Minimax,AlphaBeta
}

#[deriving(Clone)]
pub struct Ai {
    mode : Mode,
    plies : uint
}

impl Ai {
    pub fn new(mode : Mode, plies : uint) -> Ai {
        Ai{mode : mode, plies : plies }
    }

    pub fn get_move(&self, board: &Board) -> (uint, uint) {
        match self.mode {
            Simple => self.simple(board),
            Minimax => self.minimax(board),
            AlphaBeta => self.alpha_beta(board)
        }
    }

    fn simple(&self, board: &Board) -> (uint, uint) {    
       let cells = board.cells_with_mark(Empty);
       let cell = cells.get(0);
       (cell.x,cell.y)
    }

    fn minimax(&self, board: &Board) -> (uint, uint) {
        //let delegate = box TicTacToeAiDelegate as Box<MinimaxDelegate<Board,(uint,uint)>>;
        let mut scrap_board = board.clone();
        match minimax(self,&mut scrap_board,0) {
            (move,_) => move
        }
    }

    fn alpha_beta(&self, board: &Board) -> (uint, uint) {
        let delegate = box self.clone() as Box<MinimaxDelegate<Board,(uint,uint)>>;
        let mut scrap_board = board.clone();
        match minimax_alpha_beta(delegate,&mut scrap_board) {
            (move,_) => move
        }
    }

    fn score_cell(&self, board : &mut Board, cell : &Cell) -> uint{
        let directions = [(0,1),(1,0),(1,1),(1,-1)];
        directions.iter().fold(0, |a, &dir| a + self.score_cell_for_direction(board,cell,dir))
    }

    fn score_cell_for_direction(&self, board : &mut Board, cell : &Cell, dir : (int,int)) -> uint {
        let mut iters = [box board.iter(cell,dir) as Box<Iterator<&Cell>>,
                         box board.iter(cell,dir).rev() as Box<Iterator<&Cell>>];

        let iter = board.iter(cell,dir);
        iter.enumerate();

        let mark = cell.mark;

        for iter  in iters.mut_iter() {
            //let iter : &Iterator<&Cell> = iter;
            //iter.enumerate();
            // let advance_iter = iter.enumerate().advance(
            //     |(idx,cell)| idx < board.get_k() && !(cell.mark == mark || cell.mark == Empty)
            // );
            // let mark_count = advance_iter.fold(0,|a,&(idx,cell)| a);
            // for cell in iter{
            //     if cell.mark == changedCell.mark || (!breakOnEmpty && cell.mark == Empty) {
            //         count+=1;
            //     } else {
            //         break;
            //     }
            // }
        }
        0
    }
}

impl MinimaxDelegate<Board,(uint,uint)> for Ai {
    fn possible_moves<'a> (&self, current_state : &'a mut Board, depth : uint) -> Vec<(uint,uint)> {
        current_state.cells_with_mark(Empty).iter().map(|&cell| (cell.x,cell.y)).collect()
    }

    fn do_move(&self, board : & mut Board, &move : &(uint,uint), depth : uint) {
        let mark_type = if self.shouldMaximize(board,depth) { Ai } else { Player };
        board.set_mark(move,mark_type);
    }

    fn should_continue(&self, board : & mut Board, depth : uint) -> bool{
        match board.get_state() {
            PlayerWins | AiWins | CatsGame => false,
            _ => true
        }
    }

    fn undo_move(&self, board : & mut  Board, &move : &(uint,uint), depth : uint) {
        board.set_mark(move,Empty);
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
        self.plies
    }
}