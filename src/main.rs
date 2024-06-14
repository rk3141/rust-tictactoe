use std::{fmt::Display, io::stdin};

use termion::{color, style};
extern crate termion;

fn clean_term() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[derive(Copy, Clone, PartialEq)]
enum Tic {
    Nil,
    O,
    X,
}
impl Tic {
    fn next(&self) -> Tic {
        match self {
            Tic::Nil => Tic::O,
            Tic::O => Tic::X,
            Tic::X => Tic::O,
        }
    }
}
impl Display for Tic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nil => "_",
                Self::O => "O",
                Self::X => "X",
            }
        )
    }
}

struct TicTacToe {
    grid: [Tic; 9],
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            grid: [Tic::Nil; 9],
        }
    }

    fn set(&mut self, x: usize, y: usize, what: Tic) -> Result<(), Tic> {
        let pos = x + y * 3;
        if self.grid[pos] != Tic::Nil {
            Err(self.grid[pos])
        } else {
            self.grid[pos] = what;
            Ok(())
        }
    }

    fn show(&self) {
        println!(
            "{}{}  0 1 2{}",
            style::Bold,
            color::Fg(color::Red),
            style::Reset
        );
        for (ind, cell) in self.grid.iter().enumerate() {
            if ind % 3 == 0 {
                print!(
                    "{}{}{} {}",
                    style::Bold,
                    color::Fg(color::Red),
                    ind / 3,
                    style::Reset
                );
            }
            print!("{} ", cell);
            if ind % 3 == 2 {
                print!("\n");
            }
        }
    }

    fn same3(&self, a: usize, b: usize, c: usize) -> bool {
        self.grid[a] == self.grid[b] && self.grid[c] == self.grid[b] && self.grid[a] != Tic::Nil
    }

    fn digest(&self) -> bool {
        let mut win = false;
        // Horizontal Test
        for a in 0..=2 {
            let a = a * 3;
            win = win || self.same3(a, a + 1, a + 2);
        }
        // Vertical Test
        for a in 0..=2 {
            win = win || self.same3(a, a + 3, a + 6);
        }
        win = win || self.same3(0, 4, 8) || self.same3(2, 4, 6);

        win
    }
}

fn main() {
    let mut tic = TicTacToe::new();
    let mut turn = Tic::O;

    clean_term();

    loop {
        tic.show();
        if tic.digest() {
            println!(
                "{}{}{} Wins!!",
                style::Bold,
                color::Fg(color::Green),
                turn.next()
            );
            break;
        } else {
            println!("({}) Enter Row then Col: ", turn);
            let mut rowstr = String::new();
            let mut colstr = String::new();
            stdin().read_line(&mut rowstr).expect("couldnt read");
            stdin().read_line(&mut colstr).expect("couldnt read");

            let row: usize = rowstr.trim().parse().expect("Not an number");
            let col: usize = colstr.trim().parse().expect("Not an number");

            clean_term();
            let res = tic.set(col, row, turn);

            if res.is_ok() {
                turn = turn.next()
            }
        }
    }
}
