use std::int;
use board::{Board};
use board::{PlayerWins,AiWins,CatsGame};
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
}

impl MinimaxDelegate<Board,(uint,uint)> for Ai {
    fn possible_moves<'a> (&self, current_state : &'a mut Board, _depth : uint) -> Vec<(uint,uint)> {
        current_state.cells_with_mark(Empty).iter().map(|&cell| (cell.x,cell.y)).collect()
    }

    fn do_move(&self, board : & mut Board, &move : &(uint,uint), depth : uint) {
        let mark_type = if self.shouldMaximize(board,depth) { Ai } else { Player };
        board.set_mark(move,mark_type);
    }

    fn should_continue(&self, board : & mut Board, _depth : uint) -> bool{
        match board.get_state() {
            PlayerWins | AiWins | CatsGame => false,
            _ => true
        }
    }

    fn undo_move(&self, board : & mut  Board, &move : &(uint,uint), _depth : uint) {
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
            let cell_count = directions.iter().fold(0, |sum, &dir| sum + board.count_consecutive(cell,dir,false));
            score += if cell_count < board.get_k() {0} else {cell_count as int * 100}
        }

        for cell in board.cells_with_mark(Player).move_iter() {
            let cell_count = directions.iter().fold(0, |sum, &dir| sum + board.count_consecutive(cell,dir,false));
            score -= if cell_count < board.get_k() {0} else {cell_count as int * 100}
        }        

        score
    }

    fn shouldMaximize(&self, _board : & mut  Board, depth : uint) -> bool {
        depth % 2 == 0
    }

    fn max_plies(&self) -> uint {
        self.plies
    }
}