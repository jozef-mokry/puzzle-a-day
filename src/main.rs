use std::collections::HashMap;
use std::collections::HashSet;
const BOARD_ROWS: usize = 7;
const BOARD_COLS: usize = 7;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Board(u64);

impl Board {
    fn set(&mut self, row: usize, col: usize) {
        self.0 |= 1 << ((row * BOARD_COLS) + col)
    }
    fn get(&self, row: usize, col: usize) -> usize {
        self.0 as usize >> (row * BOARD_COLS + col) & 1usize
    }
    fn print(&self) {
        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                if self.get(row, col) == 1 {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    fn can_add(&self, other: &Self) -> bool {
        (self.0 & other.0) == 0
    }
    fn add(&self, other: &Self) -> Self {
        Board(self.0 | other.0)
    }

    fn remove(&self, other: &Self) -> Self {
        Board(self.0 & !other.0)
    }

    fn build_empty_board() -> Self {
        let mut start_board = Board(0);
        start_board.set(0, 6);
        start_board.set(1, 6);
        start_board.set(6, 3);
        start_board.set(6, 4);
        start_board.set(6, 5);
        start_board.set(6, 6);
        start_board
    }

    fn build_target_board(month: usize, day: usize) -> Self {
        let mut board = Self::build_empty_board();
        let (m_row, m_col) = (month / 6, month % 6);
        let (d_row, d_col) = (2 + (day / 7), day % 7);

        board.set(m_row, m_col);
        board.set(d_row, d_col);

        board.0 = !board.0;
        board.0 &= (1 << (BOARD_ROWS * BOARD_COLS)) - 1;

        board
    }
}

struct PieceBoards {
    piece: Piece,
    boards: Vec<Board>,
}

impl PieceBoards {
    fn from(piece: Piece, empty_board: &Board) -> Self {
        let mut boards = HashSet::new();

        for piece in [
            piece.clone(),
            piece.rotate_cw(),
            piece.rotate_cw().rotate_cw(),
            piece.rotate_cw().rotate_cw().rotate_cw(),
            piece.flip(),
            piece.flip().rotate_cw(),
            piece.flip().rotate_cw().rotate_cw(),
            piece.flip().rotate_cw().rotate_cw().rotate_cw(),
        ] {
            for row in 0..BOARD_ROWS {
                for col in 0..BOARD_COLS {
                    match piece.shifted(row as i64, col as i64) {
                        None => {}
                        Some(piece) => {
                            let board = piece.on_board();
                            if empty_board.can_add(&board) {
                                boards.insert(board);
                            }
                        }
                    }
                }
            }
        }

        PieceBoards {
            piece,
            boards: boards.into_iter().collect(),
        }
    }
}

#[derive(Clone)]
struct Piece {
    pos: Vec<(i64, i64)>,
    name: &'static str,
}
impl Piece {
    fn build_rectangle() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)];
        Piece {
            pos,
            name: "rectangle",
        }
    }
    fn build_z() -> Piece {
        let pos = vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)];
        Piece { pos, name: "Z" }
    }
    fn build_l() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)];
        Piece { pos, name: "L" }
    }
    fn build_t() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1)];
        Piece { pos, name: "T" }
    }
    fn build_tetris() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)];
        Piece {
            pos,
            name: "tetris",
        }
    }
    fn build_u() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)];
        Piece { pos, name: "U" }
    }
    fn build_walkie_talkie() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1)];
        Piece {
            pos,
            name: "walkie talkie",
        }
    }
    fn build_big_l() -> Piece {
        let pos = vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)];
        Piece { pos, name: "big L" }
    }

    fn on_board(&self) -> Board {
        let mut b = Board(0);
        for &(row, col) in &self.pos {
            b.set(row as usize, col as usize)
        }
        b
    }

    fn rotate_cw(&self) -> Self {
        let pos = self.pos.iter().copied().map(|(r, c)| (c, -r)).collect();
        Piece {
            pos,
            name: self.name,
        }
        .normalized()
    }

    fn flip(&self) -> Self {
        let pos = self.pos.iter().copied().map(|(r, c)| (r, -c)).collect();
        Piece {
            pos,
            name: self.name,
        }
        .normalized()
    }

    fn normalized(self) -> Self {
        let (mut min_row, mut min_col) = (None, None);
        for &(r, c) in &self.pos {
            min_row = match min_row {
                None => Some(r),
                Some(rr) if rr > r => Some(r),
                _ => min_row,
            };
            min_col = match min_col {
                None => Some(c),
                Some(cc) if cc > c => Some(c),
                _ => min_col,
            };
        }
        let (min_row, min_col) = (min_row.unwrap(), min_col.unwrap());
        Piece {
            pos: self
                .pos
                .into_iter()
                .map(|(r, c)| (r - min_row, c - min_col))
                .collect(),
            name: self.name,
        }
    }

    fn shifted(&self, row: i64, col: i64) -> Option<Piece> {
        let pos = self
            .pos
            .iter()
            .copied()
            .map(|(r, c)| {
                if r + row >= BOARD_ROWS as i64 || c + col >= BOARD_COLS as i64 {
                    None
                } else {
                    Some((r + row, c + col))
                }
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Piece {
            pos,
            name: self.name,
        })
    }

    fn build_all_pieces() -> Vec<Piece> {
        vec![
            Piece::build_rectangle(),
            Piece::build_z(),
            Piece::build_l(),
            Piece::build_t(),
            Piece::build_u(),
            Piece::build_big_l(),
            Piece::build_walkie_talkie(),
            Piece::build_tetris(),
        ]
    }
}
struct Solution(Vec<Board>);
impl Solution {
    fn print(&self) {
        let chars = ['🟥', '🟦', '🟫', '🟩', '🟧', '🟪', '🟨', '⬜'];
        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                if let Some(i) = self.0.iter().position(|board| board.get(row, col) == 1) {
                    print!("{}", chars[i]);
                } else {
                    print!("⬛");
                }
            }
            println!();
        }
        println!();
    }
}

fn get_solutions(board: &Board, parent: &HashMap<Board, Vec<Board>>) -> Vec<Solution> {
    if board.0 == 0 {
        return vec![Solution(vec![])];
    }
    match parent.get(board) {
        None => vec![],
        Some(prev_boards) => prev_boards
            .iter()
            .copied()
            .flat_map(|prev_board| {
                get_solutions(&board.remove(&prev_board), parent)
                    .into_iter()
                    .map(|mut sol| {
                        sol.0.push(prev_board);
                        sol
                    })
                    .collect::<Vec<Solution>>()
            })
            .collect(),
    }
}

fn solve(month: usize, day: usize) {
    // let's work 0-based
    let month = month - 1;
    let day = day - 1;

    let pieces = Piece::build_all_pieces();
    let mut empty_board = Board::build_empty_board();
    empty_board.set(month / 6, month % 6);
    empty_board.set(2 + (day / 7), day % 7);
    let mut all_piece_boards = vec![];
    for piece in pieces {
        all_piece_boards.push(PieceBoards::from(piece, &empty_board));
    }

    let mut options: HashSet<_> = all_piece_boards[0].boards.iter().copied().collect();
    let mut parent: HashMap<Board, Vec<Board>> = HashMap::new();
    for opt in &options {
        parent.insert(*opt, vec![*opt]);
    }
    for piece_boards in &all_piece_boards[1..] {
        let mut new_options: HashSet<_> = HashSet::new();
        for board in &piece_boards.boards {
            for option in &options {
                if option.can_add(board) {
                    let combined = option.add(board);
                    new_options.insert(combined);
                    parent.entry(combined).or_default().push(*board);
                }
            }
        }
        options = new_options;
    }

    let sols = get_solutions(&Board::build_target_board(month, day), &parent);
    for (i, sol) in sols.iter().enumerate() {
        println!("--- Solution {} ---", i + 1);
        sol.print();
    }
}

fn main() {
    solve(10, 6);
}
