use std::env::args;
use std::{fmt::Display, io::stdin};

use termion::{clear, color, style};
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

fn process_net_input(input: &String) -> (usize, usize) {
    let words: Vec<_> = input.trim().split_whitespace().collect();

    let y = words[0].parse().unwrap();
    let x = words[1].parse().unwrap();
    (x, y)
}

use std::io::{BufRead, BufReader, Error, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
fn main() -> Result<(), Error> {
    let mut tic = TicTacToe::new();

    clean_term();

    let progargs: Vec<_> = args().collect();
    let cmd = &progargs[1];

    match cmd.as_str() {
        "serve" => {
            let turn = Tic::O;

            let loopback = Ipv4Addr::new(127, 0, 0, 1);
            let socket = SocketAddrV4::new(loopback, 7692);
            let listener = TcpListener::bind(socket)?;
            let port = listener.local_addr()?;

            println!("Listening on {}, waiting for other player...", port);
            let (mut stream, addr) = listener.accept()?; //block  until requested

            println!("Connection received! {:?} is sending data.", addr);

            let mut reader = BufReader::new(stream.try_clone().unwrap());

            loop {
                clean_term();

                let mut input = String::new();
                let _ = reader.read_line(&mut input)?;
                let (x, y) = process_net_input(&input);
                if let Ok(_) = tic.set(x, y, turn.next()) {}

                if tic.digest() {
                    println!(
                        "{}{}{} Wins!!",
                        style::Bold,
                        color::Fg(color::Green),
                        turn.next()
                    );
                    break;
                }

                tic.show();

                println!("({}) Enter Row then Col: ", turn);
                let mut input = String::new();
                stdin().read_line(&mut input)?;

                let (mut x, mut y) = process_net_input(&input);
                while let Err(_) = tic.set(x, y, turn) {
                    clean_term();

                    tic.show();

                    println!("({}) Enter Row then Col: ", turn);
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;

                    (x, y) = process_net_input(&input);
                }

                clean_term();

                tic.show();

                stream.write_all(input.as_bytes())?;
                stream.flush()?;

                if tic.digest() {
                    println!("{}{}{} Wins!!", style::Bold, color::Fg(color::Green), turn);
                    break;
                }
            }

            Ok(())
        }

        "join" => {
            let turn = Tic::X;

            let addr = &progargs[2];
            let mut stream = TcpStream::connect(addr)?;
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            loop {
                tic.show();
                println!("({}) Enter Row then Col: ", turn);

                let mut input = String::new();
                stdin().read_line(&mut input)?;

                let (mut x, mut y) = process_net_input(&input);
                while let Err(_) = tic.set(x, y, turn) {
                    clean_term();

                    tic.show();

                    println!("({}) Enter Row then Col: ", turn);
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;

                    (x, y) = process_net_input(&input);
                }

                stream.write_all(input.as_bytes())?;
                stream.flush()?;

                clean_term();

                if tic.digest() {
                    println!("{}{}{} Wins!!", style::Bold, color::Fg(color::Green), turn);
                    break;
                }

                let mut input = String::new();
                let _ = reader.read_line(&mut input)?;

                let (x, y) = process_net_input(&input);
                if let Ok(_) = tic.set(x, y, turn.next()) {}

                if tic.digest() {
                    println!(
                        "{}{}{} Wins!!",
                        style::Bold,
                        color::Fg(color::Green),
                        turn.next()
                    );
                    break;
                }
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
