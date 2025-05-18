use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Settings {
    fen: String,
    pieces: HashMap<char, char>,
}

#[derive(Clone, Copy, PartialEq)]
enum Side {
    White,
    Black,
}

struct Chess {
    board: Vec<char>, // 10x10 grid including borders and newlines
    side: Side,
    pieces: HashMap<char, char>,
}

impl Chess {
    fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read and parse settings.json
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let settings: Settings = serde_json::from_str(&contents)?;

        // Parse FEN
        let fen_parts: Vec<&str> = settings.fen.split_whitespace().collect();
        if fen_parts.len() < 2 {
            return Err("Invalid FEN: missing fields".into());
        }

        // Board part (e.g., "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
        let board_fen = fen_parts[0];
        let mut board_chars = String::new();
        for c in board_fen.chars() {
            if c.is_digit(10) {
                board_chars.push_str(&".".repeat(c.to_digit(10).unwrap() as usize));
            } else if c == '/' {
                board_chars.push('\n');
            } else {
                board_chars.push(c);
            }
        }

        // Create 10x10 board with borders
        let mut board = Vec::new();
        // Top padding: two empty rows
        for _ in 0..2 {
            board.extend("         \n".chars());
        }
        // Board rows with leading space
        for line in board_chars.lines() {
            board.push(' ');
            board.extend(line.chars());
            board.push('\n');
        }
        // Bottom padding: two empty rows
        for _ in 0..2 {
            board.extend("         \n".chars());
        }

        // Side to move
        let side = match fen_parts[1] {
            "w" => Side::White,
            "b" => Side::Black,
            _ => return Err("Invalid FEN: side must be 'w' or 'b'".into()),
        };

        Ok(Chess {
            board,
            side,
            pieces: settings.pieces,
        })
    }

    fn print_board(&self) {
        let board_str: String = self
            .board
            .iter()
            .map(|&c| {
                if c == '\n' {
                    "\n".to_string()
                } else {
                    format!(" {}", self.pieces.get(&c).unwrap_or(&c))
                }
            })
            .collect();
        let side_num = match self.side {
            Side::White => 0,
            Side::Black => 1,
        };
        println!("{}\n{}", board_str, side_num);
        println!("{}", self.board.len());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chess = Chess::new("settings.json")?;
    chess.print_board();
    Ok(())
}