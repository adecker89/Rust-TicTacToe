use std::fmt;

#[deriving(Clone)]
pub enum BoardState {
    InProgress,
    PlayerWins,
    AiWins,
    CatsGame,
}

#[deriving(Eq)]
#[deriving(Clone)]
pub enum Mark {
    Player,Ai,Empty
}

impl fmt::Show for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player => write!(f.buf,"X"),
            &Ai => write!(f.buf,"O"),
            &Empty => write!(f.buf," ")
        }
    }
}

#[deriving(Clone)]
struct Cell {
    pub x : uint,
    pub y : uint,
    pub mark : Mark
}

#[deriving(Clone)]
pub struct Board {
    rows : uint,
    cols : uint,
    k : uint,   //consecutive cells needed for win
    cells : Vec<Cell>,
    state : BoardState
}

impl Board {
    pub fn new(rows : uint, cols : uint, k : uint) -> Board {
        let cells = Vec::from_fn(rows*cols,|idx|Cell{x:idx%cols, y:idx/cols , mark: Empty});
        Board{rows : rows, cols : cols, k : k, cells: cells, state : InProgress }
    }

    pub fn get_cell<'a>(&'a self, x : uint, y : uint) -> &'a Cell {
        self.cells.get(y * self.cols + x)
    }

    pub fn get_num_rows(&self) -> uint {
        self.rows
    }

    pub fn get_num_cols(&self) -> uint {
        self.cols
    }

    pub fn get_state(&self) -> BoardState {
        self.state
    }

    pub fn set_mark(& mut self, x : uint, y : uint, mark : Mark)-> BoardState {
        let cell_index = y * self.cols + x;
        self.cells.get_mut(cell_index).mark = mark;

        if(mark == Empty) {
            self.state = InProgress;
            return self.state;
        }

        let cell = self.cells.get(cell_index);
        if self.check_for_win(cell) {
            match cell.mark {
                Player => self.state = PlayerWins,
                Ai => self.state = AiWins,
                _ => fail!()
            }
        } else if self.empty_cells().len() == 0 {
            self.state = CatsGame;
        } else {
            self.state = InProgress;
        }
        self.state
    }

    pub fn empty_cells<'a>(&'a self) -> Vec<&'a Cell> {
        self.cells.iter().filter(|&cell| match cell.mark {
            Empty=>true,
            _=>false
        }).collect()
    }

    pub fn iter<'a>(&'a self, cell : &'a Cell, direction : (int,int)) -> BoardIterator<'a> {
        BoardIterator{ board : self, cell : cell, direction : direction } 
    }

    fn check_for_win(&self, changedCell : &Cell) -> bool {
        if self.check_win_for_direction(changedCell,(0,1)) { return true; }
        if self.check_win_for_direction(changedCell,(1,0)) { return true; }
        if self.check_win_for_direction(changedCell,(1,1)) {return true; }
        if self.check_win_for_direction(changedCell,(1,-1)) {return true;}
        
        false
    }
        
    fn check_win_for_direction(&self,changedCell : &Cell, direction : (int,int)) -> bool {
        let forward = box self.iter(changedCell,direction);
        let reversed = box self.iter(changedCell,direction).rev();
        let mut iters =  [forward as Box<Iterator<&Cell>>, reversed as Box<Iterator<&Cell>>];

        let mut count = 1;
        for iter in iters.mut_iter() {
            loop {
                match iter.next() {
                    Some(&cell) => {
                        if cell.mark == changedCell.mark {
                            count+=1;
                        } else {
                            break;
                        }
                    }
                    None => break
                }
            }
        }

        count == self.k
    }
}

impl fmt::Show for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "  ");
        for x in range(0,self.cols) {
            write!(f.buf, "{} ", x);
        }
        write!(f.buf, "\n");

        for (idx,cell) in self.cells.iter().enumerate() {
            if(idx % self.cols == 0) {
                write!(f.buf, "{} ", idx / self.cols);       
            }

            write!(f.buf, "{}", cell.mark);
            //println!("\ncols:{} mod:{}",self.cols,idx+1 % self.cols);
            if((idx+1) % self.cols != 0) {
                write!(f.buf,"|");
            } else if idx + 1 < self.cells.len() {
                write!(f.buf, "\n  ");
                for idx in range(0,self.cols-1) {
                    write!(f.buf, "-+");
                }
                write!(f.buf, "-\n");
            }
        }
        write!(f.buf,"")
    }
}

struct BoardIterator<'a> {
    board : &'a Board,
    cell : &'a Cell,
    direction : (int,int)
}

impl<'a> Iterator<&'a Cell> for BoardIterator<'a> {

    fn next(&mut self) -> Option<&'a Cell> {
        match self.direction {
            (x_inc,y_inc) => {
                let newy = (self.cell.y as int + y_inc) as uint;
                let newx = (self.cell.x as int + x_inc) as uint;
                if newx >= self.board.cols || newy >= self.board.rows {
                    None
                } else {
                    self.cell = self.board.get_cell(newx,newy);
                    Some(self.cell)
                }
            }
        }
    }
}

impl<'a> DoubleEndedIterator<&'a Cell> for BoardIterator<'a> {
    fn next_back(&mut self) -> Option<&'a Cell> {
        match self.direction {
            (x_inc,y_inc) => {
                let newy = (self.cell.y as int - y_inc) as uint;
                let newx = (self.cell.x as int - x_inc) as uint;
                if newx >= self.board.cols || newy >= self.board.rows {
                    None
                } else {
                    self.cell = self.board.get_cell(newx,newy);
                    Some(self.cell)
                }
            }
        }
    }
}