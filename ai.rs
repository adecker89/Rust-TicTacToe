use std::int;
use std::cmp;
use std::fmt;

pub trait MinimaxDelegate<S,M> {
    fn possible_moves<'a> (&self, current_state : &'a mut S, depth : uint) -> Vec<M>;
    fn do_move(&self, state : & mut  S, move : &M, depth : uint);
    fn undo_move(&self, state : & mut  S, move : &M, depth : uint);
    fn should_continue(&self, state : & mut  S, depth : uint) -> bool;
    fn score(&self, state : & mut  S, depth : uint) -> int;
    fn shouldMaximize(&self, state : & mut  S, depth : uint) -> bool;
    fn max_plies(&self) -> uint;
}

pub fn minimax<'a,S,M>(delegate: &MinimaxDelegate<S,M>, state : &'a mut S, depth : uint) -> (M, int) {
    let mut moves = delegate.possible_moves(state,depth);
    let mut scores = Vec::with_capacity(moves.len());

    if moves.len() == 0 {
        fail!("should_continue must return false before all possible moves are exhausted");
    }

    for move in moves.iter() {
        delegate.do_move(state,move,depth);
        if !delegate.should_continue(state,depth) || depth+1 == delegate.max_plies() {
            //base case
            scores.push(delegate.score(state,depth+1));
        } else {
            //recursive case
            let move_score = minimax(delegate,state,depth+1);
            match move_score {(_,score) => scores.push(score)};
        }
        delegate.undo_move(state,move,depth);
    }

    println!("depth:{} scores:{}",depth,scores);
    let score_pair = if delegate.shouldMaximize(state,depth) {
        scores.iter().enumerate().max_by(|&(_,x)| x).unwrap()
    } else {
        scores.iter().enumerate().min_by(|&(_,x)| x).unwrap()
    };

    match score_pair {
        (idx,score) => (moves.swap_remove(idx).unwrap(),score.clone())
    }
}
pub fn minimax_alpha_beta<'a,S,M  : fmt::Show>(delegate: &MinimaxDelegate<S,M>, state : &'a mut S)  -> (M, int) {
    minimax_alpha_beta_helper(delegate,state,0,int::MIN,int::MAX)
}

fn minimax_alpha_beta_helper<'a,S,M : fmt::Show>(delegate: &MinimaxDelegate<S,M>, state : &'a mut S, depth : uint, alpha : int, beta : int) -> (M, int) {
    let mut moves = delegate.possible_moves(state,depth);
    

    if moves.len() == 0 {
        fail!("should_continue must return false before all possible moves are exhausted");
    }

    let mut a = alpha;
    let mut b = beta;
    let mut best_move_idx = 0;
    let maximizing = delegate.shouldMaximize(state,depth);

    {
        let mut scores = Vec::with_capacity(moves.len());
        for (idx,move) in moves.iter().enumerate() {
            delegate.do_move(state,move,depth);

            let score = if !delegate.should_continue(state,depth) || depth+1 == delegate.max_plies() {
                //base case
                delegate.score(state,depth+1)
            } else {
                //recursive case
                let move_score = minimax_alpha_beta_helper(delegate,state,depth+1,a,b);
                match move_score { (_,score) => score }
            };

            scores.push((move,score));
                           
            if maximizing {
                if score > a {
                    a = score;
                    best_move_idx = idx;
                }
            } else {
                if score < b {
                    b = score;
                    best_move_idx = idx;
                }
            }

            delegate.undo_move(state,move,depth);

            if a >= b {
                //if depth < 2 {println!("pruning depth:{} after {} subtrees",depth,idx);}
                break;
            }
        }
        if depth < 2 {println!("depth:{} scores:{} move:{}",depth,scores,moves.get(best_move_idx));}
    }
    if maximizing {
        (moves.swap_remove(best_move_idx).unwrap(),a)
    } else {
        (moves.swap_remove(best_move_idx).unwrap(),b)
    }
}