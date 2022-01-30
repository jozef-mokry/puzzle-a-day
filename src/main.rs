use std::collections::HashMap;
use std::collections::HashSet;
const BOARD_ROWS: usize = 7;
const BOARD_COLS: usize = 7;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Board(u64);

impl Board {
    fn set(self: &mut Self, row: usize, col: usize) {
        self.0 |= 1 << ((row * BOARD_COLS) + col)
    }
    fn get(self: &Self, row: usize, col: usize) -> usize {
        self.0 as usize >> (row * BOARD_COLS + col) & 1usize
    }
    fn print(self: &Self) {
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
    fn can_add(self: &Self, other: &Self) -> bool {
        (self.0 & other.0) == 0
    }
    fn add(self: &Self, other: &Self) -> Self {
        Board(self.0 | other.0)
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

    fn on_board(self: &Self) -> Board {
        let mut b = Board(0);
        for &(row, col) in &self.pos {
            b.set(row as usize, col as usize)
        }
        b
    }

    fn rotate_cw(self: &Self) -> Self {
        let pos = self.pos.iter().copied().map(|(r, c)| (c, -r)).collect();
        Piece {
            pos,
            name: self.name,
        }
        .normalized()
    }

    fn flip(self: &Self) -> Self {
        let pos = self.pos.iter().copied().map(|(r, c)| (r, -c)).collect();
        Piece {
            pos,
            name: self.name,
        }
        .normalized()
    }

    fn normalized(self: Self) -> Self {
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

    fn shifted(self: &Self, row: i64, col: i64) -> Option<Piece> {
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

fn solve(month: usize, day: usize) {
    // build start board
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
    for piece_boards in &all_piece_boards[1..] {
        let mut new_options: HashSet<_> = HashSet::new();
        for board in &piece_boards.boards {
            for option in &options {
                if option.can_add(board) {
                    let combined = option.add(board);
                    new_options.insert(combined);
                    parent.entry(combined).or_default().push(option.clone());
                }
            }
        }
        options = new_options;
    }

    assert!(options.len() == 1);
    fn count_solutions(board: &Board, parent: &HashMap<Board, Vec<Board>>) -> usize {
        match parent.get(board) {
            None => 1,
            Some(boards) => boards.iter().map(|x| count_solutions(x, parent)).sum(),
        }
    }
    let solution_count = count_solutions(&options.into_iter().next().unwrap(), &parent);
    println!(
        "Day {} Month {} has {} solutions",
        day, month, solution_count
    );
    // for (i, mut option) in options.iter().enumerate() {
    //     println!("Solution {}", i);
    //     loop {
    //         option.print();
    //         match parent.get(option) {
    //             None => {
    //                 break;
    //             }
    //             Some(opt) => {
    //                 option = opt;
    //             }
    //         }
    //     }
    // }
}

fn main() {
    for month in 0..12 {
        for day in 0..31 {
            solve(month, day);
        }
    }
}
