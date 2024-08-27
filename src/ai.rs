use crate::{bits::BitBoard, game::Game};
use pyo3::prelude::*;
use rand::prelude::*;

pub trait Bot {
    /// Find the next move (an int between 0 and 63)
    /// Return -1 if there is no legal move
    fn find_move(&self, board: &Game) -> i32;
}

#[pyclass]
pub struct AlphaBetaBot {
    #[pyo3(get)]
    depth: usize,

    // Do exhaustive search when there is less than exhaustive_depth empty square
    #[pyo3(get)]
    exhaustive_depth: usize,
}

impl Bot for AlphaBetaBot {
    fn find_move(&self, g: &Game) -> i32 {
        let count = (g.board.0 + g.board.1).count_ones() as usize;
        let depth = if count > 64 - self.exhaustive_depth {
            100
        } else {
            self.depth
        };
        let (_, move_) = self.do_search(
            &AlphaBetaEval { count },
            &mut rand::thread_rng(),
            &g.board,
            depth,
            -i32::MAX,
            i32::MAX,
        );
        return move_;
    }
}

#[pymethods]
impl AlphaBetaBot {
    #[new]
    pub fn new(depth: usize, exhaustive_depth: usize) -> Self {
        Self {
            depth,
            exhaustive_depth,
        }
    }

    #[pyo3(name = "find_move")]
    fn run(&self, board: &Game) -> i32 {
        self.find_move(board)
    }
}

impl AlphaBetaBot {
    fn do_search(
        &self,
        eval: &AlphaBetaEval,
        rng: &mut ThreadRng,
        board: &BitBoard,
        depth: usize,
        mut alpha: i32,
        beta: i32,
    ) -> (i32, i32) {
        if depth == 0 {
            return (eval.evaluate(board), -1);
        }
        let mut moves = board.available_moves_list();
        if moves.is_empty() {
            let board = board.pass_move();
            let moves = board.available_moves();
            if moves == 0 {
                return (-eval.final_evaluate(&board), -1);
            }
            let (score, _) = self.do_search(eval, rng, &board, depth - 1, -beta, -alpha);
            return (-score, -1);
        }
        moves.shuffle(rng);
        let mut best_move = moves[0];
        for move_ in moves {
            if alpha >= beta {
                break;
            }
            let (score, _) = self.do_search(
                eval,
                rng,
                &board.make_move(move_).unwrap(),
                depth - 1,
                -beta,
                -alpha,
            );
            if -score > alpha {
                alpha = -score;
                best_move = move_
            }
        }
        (alpha, best_move as i32)
    }
}

struct AlphaBetaEval {
    // Number of pieces at the beginning of search
    count: usize,
}

impl AlphaBetaEval {
    fn evaluate(&self, board: &BitBoard) -> i32 {
        let scorer = |mask: u64| {
            (0..64)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| Self::POSITION_SCORES[i])
                .sum::<i32>()
        };
        let n_moves0 = board.available_moves().count_ones() as i32;
        let n_moves1 = board.pass_move().available_moves().count_ones() as i32;
        let mut score = scorer(board.0) - scorer(board.1) + 10 * n_moves0 - 10 * n_moves1;
        if self.count > 54 {
            let (cnt0, cnt1) = board.count();
            score += 2 * (self.count as i32 - 54) * (cnt0 - cnt1) as i32
        }
        score
    }

    fn final_evaluate(&self, board: &BitBoard) -> i32 {
        let (sc1, sc2) = board.count();
        if sc1 < sc2 {
            -i32::MAX + (64 + sc1 - sc2)
        } else if sc1 > sc2 {
            i32::MAX - (64 + sc1 - sc2)
        } else {
            0
        }
    }

    #[rustfmt::skip]
    const POSITION_SCORES: [i32; 64] = [
        300, -40,  20,   5,   5,  20, -40, 300,
        -40, -80,  -5,  -5,  -5,  -5, -80, -40,
        20,  -5,  15,   1,   1,  15,  -5,  20,
        5,  -5,   1,   1,   1,   1,  -5,   5,
        5,  -5,   1,   1,   1,   1,  -5,   5,
        20,  -5,  15,   1,   1,  15,  -5,  20,
        -40, -80,  -5,  -5,  -5,  -5, -80, -40,
        300, -40,  20,   5,   5,  20, -40, 300,
    ];
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    use super::{AlphaBetaBot, Bot};

    #[test]
    fn test() {
        let mut b = Game::default();
        let ai = [AlphaBetaBot::new(3, 0), AlphaBetaBot::new(6, 10)];

        while !b.state.ended {
            let pos = ai[b.current_player].find_move(&b);
            if pos < 0 {
                b.pass_move();
                println!("PASS");
            } else {
                b.make_move(pos as usize);
                println!("{}\n---\n", b.__repr__());
            }
        }
        assert!(b.state.black_score > b.state.white_score)
    }
}
