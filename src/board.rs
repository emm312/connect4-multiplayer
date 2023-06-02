use std::fmt::Display;

use termcolor::{StandardStream, ColorChoice, WriteColor, ColorSpec, Color};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    X,
    O,
    Empty,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " ")?,
            
            Self::O => { yellow(); write!(f, "⏺")?; default(); },
            Self::X => { red(); write!(f, "⏺")?; default(); },
        };
        Ok(())
    }
}

fn default() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&ColorSpec::new().set_fg(None)).unwrap();
}

fn yellow() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
}

fn red() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(&ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
}

pub struct Board {
    board: Vec<Vec<State>>,
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: vec![vec![State::Empty; 7]; 6],
        }
    }

    pub fn place_obj(&mut self, piece: State, col: usize) -> Result<(), String> {
        for i in 0..self.board.len() {
            if self.board[i][col] != State::Empty {
                if i != 0 {
                    self.board[i - 1][col] = piece;
                    return Ok(());
                } else {
                    return Err(String::from("That column is full."));
                }
            }
        }
        self.board[5][col] = piece;
        return Ok(());
    }

    pub fn wincheck(&self) -> Option<State> {
        // horizontal
        for row in 0..self.board.len() {
            for pos in 0..self.board[row].len() - 3 {
                let col = self.board[row][pos];
                if col == State::Empty {
                    continue;
                }
                if self.board[row][pos] == col
                    && self.board[row][pos + 1] == col
                    && self.board[row][pos + 2] == col
                    && self.board[row][pos + 3] == col
                {
                    return Some(col);
                }
            }
        }

        // vertical
        for row in 0..self.board.len() - 3 {
            for pos in 0..self.board[row].len() {
                let col = self.board[row][pos];
                if col == State::Empty {
                    continue;
                }
                if self.board[row][pos] == col
                    && self.board[row + 1][pos] == col
                    && self.board[row + 2][pos] == col
                    && self.board[row + 3][pos] == col
                {
                    return Some(col);
                }
            }
        }
        // asc diag
        for row in 3..self.board.len() {
            for pos in 0..self.board[0].len() - 3 {
                let col = self.board[row][pos];
                if col == State::Empty {
                    continue;
                }
                if self.board[row][pos] == col
                    && self.board[row - 1][pos + 1] == col
                    && self.board[row - 2][pos + 2] == col
                    && self.board[row - 3][pos + 3] == col
                {
                    return Some(col);
                }
            }
        }

        // dsc diag
        for row in 3..self.board.len() {
            for pos in 3..self.board[0].len() {
                let col = self.board[row][pos];
                if col == State::Empty {
                    continue;
                }
                if self.board[row][pos] == col
                    && self.board[row - 1][pos - 1] == col
                    && self.board[row - 2][pos - 2] == col
                    && self.board[row - 3][pos - 3] == col
                {
                    return Some(col);
                }
            }
        }

        None
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter() {
            for (pos, col) in row.iter().enumerate() {
                if pos != 0 {
                    write!(f, " ┃ {}", col)?;
                } else {
                    write!(f, "┃ {}", col)?;
                }
            }
            writeln!(f, " ┃")?;
        }
        writeln!(f, "┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫")?;
          write!(f, "┃ 1 ┃ 2 ┃ 3 ┃ 4 ┃ 5 ┃ 6 ┃ 7 ┃")?;
        Ok(())
    }
}
