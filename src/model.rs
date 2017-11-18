// ------------- Game Rules -------------- //

pub struct Game {
    board: Board,
    turn_player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            turn_player: Player::P1,
        }
    }

    pub fn play(&mut self, position: Position) -> Option<Player> {
        if let None = self.board.state[position.row][position.column] {
            self.board.state[position.row][position.column] = Some(self.turn_player);
            self.turn_player = self.turn_player.other();
            self.board.state[position.row][position.column]
        }
        else {
            None
        }
    }

    pub fn result(&self, last_move: Position) -> Option<GameResult> {
        match self.board.lines_of_pos(last_move).iter()
            .map(|l| l.all_same_player())
            .fold(None, |a, p| a.or(p)) {
                Some(p) => Some(p.win_result()),
                None => None
            }
    }

    pub fn turn_player(&self) -> Player {
        self.turn_player
    }
}

// ----------- Board and Line ------------ //

pub const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        if row >= BOARD_SIZE || column >= BOARD_SIZE {
            panic!("Invalid position: BOARD_SIZE is {} but position is ({}, {})", BOARD_SIZE, row, column);
        }
        Position { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

struct Board {
    state: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Board {
        Board { state: [[None; BOARD_SIZE]; BOARD_SIZE] }
    }

    fn lines_of_pos(&self, pos: Position) -> [Line; 4] {
        let row = Line { state: self.state[pos.row] };
        let column = Line::new_from_vec(self.state.iter().map(|r| r[pos.column]).collect());
        let diagonal = Line::new_from_vec(self.state.iter().enumerate().map(|(i, r)| r[i]).collect());
        let antidiagonal = Line::new_from_vec(self.state.iter().enumerate().map(|(i, r)| r[BOARD_SIZE-1-i]).collect());
        [row, column, diagonal, antidiagonal]
    }
}

struct Line {
    state: [Option<Player>; BOARD_SIZE],
}

impl Line {
    fn new_from_vec(line: Vec<Option<Player>>) -> Line {
        if line.len() != BOARD_SIZE {
            panic!("Total size of line mismatch: BOARD_SIZE is {} but total size is {}", BOARD_SIZE, line.len());
        }
        let mut state: [Option<Player>; 3] = [None; 3];
        for i in 0..BOARD_SIZE {
            state[i] = line[i];
        }
        Line { state }
    }

    fn all_same_player(&self) -> Option<Player> {
        if self.state.iter().all(|p| *p == self.state[0]) {
            self.state[0]
        }
        else {
            None
        }
    }
}

// -------- Player and GameResult -------- //

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Player {
    P1,
    P2,
}

#[derive(Copy, Clone)]
pub enum GameResult {
    P1Wins,
    P2Wins,
    Draw,
}

impl Player {
    fn other(self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }

    fn win_result(self) -> GameResult {
        match self {
            Player::P1 => GameResult::P1Wins,
            Player::P2 => GameResult::P2Wins,
        }
    }
}

impl GameResult {
    pub fn win_player(self) -> Option<Player> {
        match self {
            GameResult::P1Wins => Some(Player::P1),
            GameResult::P2Wins => Some(Player::P2),
            _ => None,
        }
    }
}

