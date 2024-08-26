use crate::{bits::BitBoard, board::Board, consts::ALPHA_BETA_SCORES};
use pyo3::prelude::*;
use rand::prelude::*;

pub trait AI {
    fn run(&self, board: &Board) -> usize;
}

#[pyclass]
pub struct AlphaBeta {
    depth: usize,
}

impl AI for AlphaBeta {
    fn run(&self, board: &Board) -> usize {
        let (_, move_) = self.do_search(
            &mut rand::thread_rng(),
            &board.board,
            self.depth,
            -i32::MAX,
            i32::MAX,
        );
        return move_;
    }
}

#[pymethods]
impl AlphaBeta {
    #[new]
    pub fn new(depth: usize) -> Self {
        Self { depth }
    }

    fn find_move(&self, board: &Board) -> usize {
        self.run(board)
    }
}

impl AlphaBeta {
    fn do_search(
        &self,
        rng: &mut ThreadRng,
        board: &BitBoard,
        depth: usize,
        mut alpha: i32,
        beta: i32,
    ) -> (i32, usize) {
        if depth == 0 {
            return (self.evaluate(board), 0);
        }
        let mut moves = board.available_moves_list();
        if moves.is_empty() {
            let board = board.pass_move();
            let moves = board.available_moves();
            if moves == 0 {
                return (self.final_evaluate(&board), 0);
            }
            let (score, _) = self.do_search(rng, &board, depth - 1, -beta, -alpha);
            return (-score, 0);
        }
        moves.shuffle(rng);
        let mut best_move = moves[0];
        for move_ in moves {
            if alpha >= beta {
                break;
            }
            let (score, _) = self.do_search(
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
        (alpha, best_move)
    }

    fn evaluate(&self, board: &BitBoard) -> i32 {
        // let (sc1, sc2) = board.count();
        // if sc1 + sc2 > 54 {
        //     return 5 * (sc1 - sc2);
        // }
        let scorer = |mask: u64| {
            (0..64)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| ALPHA_BETA_SCORES[i])
                .sum::<i32>()
        };
        let n_moves0 = board.available_moves().count_ones() as i32;
        let n_moves1 = board.pass_move().available_moves().count_ones() as i32;
        scorer(board.0) - scorer(board.1) + 10 * n_moves0 - 10 * n_moves1
    }

    fn final_evaluate(&self, board: &BitBoard) -> i32 {
        let (sc1, sc2) = board.count();
        if sc1 < sc2 {
            -i32::MAX
        } else if sc1 > sc2 {
            i32::MAX
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bits::BitBoard, board::Board};

    use super::{AlphaBeta, AI};

    #[test]
    fn test() {
        let mut b = Board::default();
        let ai = [AlphaBeta { depth: 5 }, AlphaBeta { depth: 6 }];

        for i in 0..80 {
            if b.available_moves().is_empty() {
                b.pass_move();
                println!("PASS");
            } else {
                let move_ = ai[i % 2].run(&b);
                let state = b.make_move(move_);
                println!("{}\n---\n", b.__repr__());
                if state.ended {
                    break;
                }
            }
        }
        assert!(false);
    }

    #[test]
    fn test_evaluate() {
        let ai = AlphaBeta { depth: 3 };
        let b = BitBoard(3, 12);
        println!("{:?}", b);
        assert_eq!(ai.evaluate(&BitBoard(3, 12)), 245);
    }
}
