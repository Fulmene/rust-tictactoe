// ------------- Game Rules -------------- //

pub struct Game {
    board: Board,
    turn_player: Player,
}

// ----------- Board and Line ------------ //

pub const BOARD_SIZE: usize = 3;

struct Board {
    state: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn empty_board() -> Board {
        Board { state: [[None; BOARD_SIZE]; BOARD_SIZE] }
    }
}

struct Line {
    state: [Option<Player>; BOARD_SIZE],
}

// -------- Player and GameResult -------- //

#[derive(Copy, Clone)]
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
    fn get_other(self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }

    fn get_win_result(self) -> GameResult {
        match self {
            Player::P1 => GameResult::P1Wins,
            Player::P2 => GameResult::P2Wins,
        }
    }
}

