use crate::bits::BitBoard;
use pyo3::prelude::*;
use std::cmp::Ordering::*;

const PLAYERS: [char; 2] = ['B', 'W'];

#[pyclass]
#[derive(Clone)]
pub struct Board {
    pub board: BitBoard,
    pub current_player: usize, // 0 or 1
    #[pyo3(get)]
    pub state: State,
}

#[pyclass(get_all)]
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub player: char,
    pub ended: bool,
    pub black_score: i32,
    pub white_score: i32,
    pub cells: Vec<char>,
    pub can_move: bool,
}

#[pymethods]
impl State {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

#[pymethods]
impl Board {
    #[staticmethod]
    pub fn default() -> Self {
        let board = BitBoard::default();
        let state = Self::compute_state(&board, 0);
        Self {
            board,
            current_player: 0,
            state,
        }
    }

    pub fn available_moves(&self) -> Vec<usize> {
        let mask = self.board.available_moves();
        (0..64).filter(|i| mask >> i & 1 == 1).collect()
    }

    pub fn pass_move(&mut self) -> State {
        self.board = self.board.pass_move();
        self.current_player = 1 - self.current_player;
        self.state = Self::compute_state(&self.board, self.current_player);
        self.state.clone()
    }

    pub fn make_move(&mut self, place: usize) -> State {
        let next = self.board.make_move(place).unwrap();
        self.current_player = 1 - self.current_player;
        self.board = next;
        self.state = Self::compute_state(&self.board, self.current_player);
        self.state.clone()
    }

    pub fn __repr__(&self) -> String {
        let state = &self.state;
        let mut s = String::new();
        for (i, &c) in state.cells.iter().enumerate() {
            if i % 8 == 0 {
                s.push('\n')
            }
            s.push(c);
        }
        if state.ended {
            s.push_str(match state.black_score.cmp(&state.white_score) {
                Equal => "Game draw!",
                Less => "White won!",
                Greater => "Black won!",
            });
        } else {
            s.push_str(&format!(
                "\n{} to play. Available moves: {:?}",
                PLAYERS[self.current_player],
                self.available_moves()
            ));
        }
        s
    }
}

impl Board {
    fn compute_state(board: &BitBoard, current_player: usize) -> State {
        let (cnt0, cnt1) = board.count();
        let moves = board.available_moves();
        let cells: Vec<_> = (0..64)
            .map(
                |i| match (board.0 >> i & 1, board.1 >> i & 1, moves >> i & 1) {
                    (1, 0, 0) => PLAYERS[current_player],
                    (0, 1, 0) => PLAYERS[1 - current_player],
                    (0, 0, 1) => '?',
                    (0, 0, 0) => '.',
                    (_, _, _) => unreachable!(),
                },
            )
            .collect();

        let ended = moves == 0 && board.pass_move().available_moves() == 0;
        let player = PLAYERS[current_player];
        State {
            player,
            ended,
            black_score: if player == 'B' { cnt0 } else { cnt1 },
            white_score: if player == 'W' { cnt0 } else { cnt1 },
            cells,
            can_move: moves != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn default_test() {
        let mut b = Board::default();

        assert_eq!(b.available_moves(), &[19, 26, 37, 44]);
        b.make_move(44);
        // assert_eq!(b.make_move(44), new(44, 'B', vec![36], 4, 1));
        assert_eq!(b.current_player, 1);

        assert_eq!(b.available_moves(), &[29, 43, 45]);
        b.make_move(29);
        // assert_eq!(b.make_move(29), new(29, 'W', vec![28], 3, 3));
        assert_eq!(b.current_player, 0);

        assert_eq!(b.__repr__(), "\n........\n........\n..?????.\n...WWW..\n...BB...\n....B...\n........\n........\nB to play. Available moves: [18, 19, 20, 21, 22]");
    }
}
