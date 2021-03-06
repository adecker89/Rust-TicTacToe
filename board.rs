use std::fmt;

#[deriving(Eq)]
#[deriving(Clone)]
pub enum BoardState {
    InProgress,
    PlayerWins,
    AiWins,
    CatsGame,
}

impl fmt::Show for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PlayerWins => write!(f,"Player Wins!"),
            &AiWins => write!(f,"Ai Wins!"),
            &CatsGame => write!(f,"Cats Game"),
            &InProgress => write!(f,"Game still in progress"),
        }
    }
}

#[deriving(Eq)]
#[deriving(Clone)]
pub enum Mark {
    Player,Ai,Empty
}

impl fmt::Show for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player => write!(f,"X"),
            &Ai => write!(f,"O"),
            &Empty => write!(f," ")
        }
    }
}

#[deriving(Clone)]
pub struct Cell {
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
    last_changed_cell : Option<uint>
}

impl Board {
    pub fn new(rows : uint, cols : uint, k : uint) -> Board {
        let cells = Vec::from_fn(rows*cols,|idx|Cell{x:idx%cols, y:idx/cols , mark: Empty});
        Board{rows : rows, cols : cols, k : k, cells: cells, last_changed_cell : None }
    }

    pub fn get_cell<'a>(&'a self, x : uint, y : uint) -> &'a Cell {
        if x >= self.cols || y >= self.rows {fail!("index out of bounds")};
        self.cells.get(y * self.cols + x)
    }

    pub fn get_num_rows(&self) -> uint {
        self.rows
    }

    pub fn get_num_cols(&self) -> uint {
        self.cols
    }

    pub fn get_k(&self) -> uint {
        self.k
    }

    pub fn get_state(&self) -> BoardState {
        let cell_idx = match self.last_changed_cell {
            None => {return InProgress;},
            Some(x) => x
        };            

        let cell = self.cells.get(cell_idx);
        if cell.mark == Empty {
            //We just erased a move, so we'll assume the game is in progress
            InProgress
        } else if self.check_for_win(cell) {
            match cell.mark {
                Player => PlayerWins,
                Ai => AiWins,
                _ => fail!("Unknown player won")
            }
        } else if self.cells_with_mark(Empty).len() == 0 {
            CatsGame
        } else {
            InProgress
        }
    }

    pub fn set_mark<'a>(& mut self, (x,y) : (uint,uint), mark : Mark) {
        if x >= self.cols || y >= self.rows {fail!("index out of bounds")};
        let cell_idx = y * self.cols + x;        
        self.cells.get_mut(cell_idx).mark = mark;        
        self.last_changed_cell = Some(cell_idx);
    }

    pub fn check_valid_move(&self, move : (uint,uint)) -> bool {
        match move {
            (x,y) => {
                if x >= self.get_num_cols() || y >= self.get_num_rows() {
                    return false;
                }
                self.get_cell(x,y).mark == Empty
            }
        }
    }

    pub fn cells_with_mark<'a>(&'a self, mark : Mark) -> Vec<&'a Cell> {
        self.cells.iter().filter(|&cell| cell.mark == mark).collect()
    }

    pub fn iter<'a>(&'a self, cell : &'a Cell, direction : (int,int)) -> BoardIterator<'a> {
        BoardIterator{ board : self, cell : cell, direction : direction } 
    }

    fn check_for_win(&self, changedCell : &Cell) -> bool {
       self.max_consecutive(changedCell) >= self.k
    }

    pub fn max_consecutive(&self,changedCell : &Cell) -> uint {
        let directions = [(0,1),(1,0),(1,1),(1,-1)];
        directions.iter().map(|&dir| self.count_consecutive(changedCell,dir,true)).max_by(|&x| x).unwrap()
    }
        
    pub fn count_consecutive(&self,changedCell : &Cell, direction : (int,int), breakOnEmpty : bool) -> uint {
        let forward = box self.iter(changedCell,direction);
        let reversed = box self.iter(changedCell,direction).rev();
        let mut iters =  [forward as Box<Iterator<&Cell>>, reversed as Box<Iterator<&Cell>>];

        let mut count = 1;
        for mut iter in iters.mut_iter() {
            for cell in iter {
                if cell.mark == changedCell.mark || (!breakOnEmpty && cell.mark == Empty) {
                    count+=1;
                } else {
                    break;
                }
            }
        }

        count
    }
}

impl fmt::Show for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = "".to_owned();
        result = result + "  ";

        for x in range(0,self.cols) {
            result = result + format!("{} ", x);
        }
        result = result + format!("\n");

        for (idx,cell) in self.cells.iter().enumerate() {
            if idx % self.cols == 0 {
                result = result + format!("{} ", idx / self.cols);       
            }

            result = result + format!("{}", cell.mark);
            if (idx+1) % self.cols != 0 {
                result = result + "|";
            } else if idx + 1 < self.cells.len() {
                result = result + format!("\n  ");
                for _ in range(0,self.cols-1) {
                    result = result + format!("-+");
                }
                result = result + format!("-\n");
            }
        }
        f.write(result.into_bytes())
    }
}

struct BoardIterator<'a> {
    board : &'a Board,
    cell : &'a Cell,
    direction : (int,int)
}

impl<'a> Iterator<&'a Cell> for BoardIterator<'a> {
    #[inline]
    fn next(&mut self) -> Option<&'a Cell> {
        match self.direction {
            (x_inc,y_inc) => {
                if x_inc == 0 && y_inc == 0 { fail!{"Invalid direction"} };
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

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, Some(self.board.cells.len()))
    }
}

impl<'a> DoubleEndedIterator<&'a Cell> for BoardIterator<'a> {
    #[inline]
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

#[test]
fn test_count_consecutive() {
    let mut board = Board::new(5,5,4);
    board.set_mark((0,0),Ai);
    assert!(board.count_consecutive(board.get_cell(0,0),(0,1),true)==1);

    board.set_mark((1,0),Ai);
    assert!(board.count_consecutive(board.get_cell(0,0),(1,0),true)==2);
    assert!(board.count_consecutive(board.get_cell(0,0),(0,1),true)==1);

    board.set_mark((1,1),Ai);
    board.set_mark((2,2),Ai);
    assert!(board.count_consecutive(board.get_cell(0,0),(1,1),true)==3);
    assert!(board.count_consecutive(board.get_cell(1,0),(0,1),true)==2);

    board.set_mark((0,4),Ai);
    board.set_mark((1,3),Ai);
    assert!(board.count_consecutive(board.get_cell(0,4),(1,-1),true)==3);

    board.set_mark((1,4),Ai);
    board.set_mark((2,4),Ai);
    board.set_mark((3,4),Ai);
    board.set_mark((4,4),Ai);
    assert!(board.count_consecutive(board.get_cell(0,4),(1,0),true)==5);
}


#[test]
fn test_win() {
    let mut board = Board::new(5,5,4);
    board.set_mark((0,0),Player);
    board.set_mark((1,1),Player);
    board.set_mark((2,2),Player);
     board.set_mark((3,3),Player);

    assert!(board.get_state() == PlayerWins);
}