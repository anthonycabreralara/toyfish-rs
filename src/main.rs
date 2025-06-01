use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
struct Settings {
    fen: String,
    pieces: HashMap<char, char>,
    colors: HashMap<char, i32>,
    directions: HashMap<char, Vec<i32>>,
    rank_2: Vec<i32>,
    rank_7: Vec<i32>,
}

#[derive(Clone, Copy, PartialEq)]
enum Side {
    White,
    Black,
}

#[derive(Debug)]
struct Move {
    source: usize,
    target: usize,
    piece: char,
    captured_piece: char,
}

struct Chess {
    board: Vec<char>,
    side: Side,
    pieces: HashMap<char, char>,
    colors: HashMap<char, i32>,
    directions: HashMap<char, Vec<i32>>,
    rank_2: Vec<i32>,
    rank_7: Vec<i32>,
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
            colors: settings.colors,
            directions: settings.directions,
            rank_2: settings.rank_2,
            rank_7: settings.rank_7,
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
    }

    fn generate_moves(&self) -> Vec<Move> {
        let mut move_list: Vec<Move> = Vec::new();

        // Helper function to check if a square is within the 8x8 board
        fn is_valid_square(index: i32) -> bool {
            // Playable board is from index 21 (a1) to 98 (h8), excluding borders
            if index < 21 || index > 98 {
                return false;
            }
            // Check if the square is in the playable 8x8 area (columns 1-8)
            let col = index % 10;
            col >= 1 && col <= 8
        }

        for i in 0..self.board.len() {
            let piece = self.board[i];

            // Skip non-piece characters
            if matches!(piece, ' ' | '.' | '\n') {
                continue;
            }

            // Get piece color and verify side
            let piece_side = match self.colors.get(&piece) {
                Some(&color) => color,
                None => continue, // Skip if piece not in colors map
            };

            let piece_side_enum = match piece_side {
                0 => Side::White,
                1 => Side::Black,
                _ => continue,
            };

            if piece_side_enum != self.side {
                continue;
            }

            for &offset in self.directions[&piece].iter() {
                let mut target_square = i as i32 + offset;

                // Handle pawn special cases separately
                if matches!(piece, 'P' | 'p') {
                    let is_white = piece == 'P';
                    let forward = if is_white { -10 } else { 10 };
                    let double_forward = if is_white { -20 } else { 20 };
                    let captures = if is_white { [-11, -9] } else { [9, 11] };

                    // Single forward move
                    if offset == forward && is_valid_square(target_square) {
                        let captured_piece = self.board[target_square as usize];
                        if captured_piece == '.' {
                            move_list.push(Move {
                                source: i,
                                target: target_square as usize,
                                piece,
                                captured_piece,
                            });
                        }
                    }

                    // Double forward move
                    if offset == double_forward && is_valid_square(target_square) {
                        let start_rank = if is_white { &self.rank_2 } else { &self.rank_7 };
                        let intermediate = i as i32 + forward;
                        if start_rank.contains(&(i as i32))
                            && self.board[intermediate as usize] == '.'
                            && self.board[target_square as usize] == '.'
                        {
                            move_list.push(Move {
                                source: i,
                                target: target_square as usize,
                                piece,
                                captured_piece: '.',
                            });
                        }
                    }

                    // Captures
                    if captures.contains(&offset) && is_valid_square(target_square) {
                        let captured_piece = self.board[target_square as usize];
                        if captured_piece != '.'
                            && captured_piece != ' '
                            && captured_piece != '\n'
                        {
                            if let Some(&captured_side) = self.colors.get(&captured_piece) {
                                if captured_side != piece_side {
                                    move_list.push(Move {
                                        source: i,
                                        target: target_square as usize,
                                        piece,
                                        captured_piece,
                                    });
                                }
                            }
                        }
                    }

                    continue; // Skip further processing for pawns
                }

                // Non-pawn pieces (sliding and non-sliding)
                loop {
                    if !is_valid_square(target_square) {
                        break;
                    }

                    let captured_piece = self.board[target_square as usize];
                    if matches!(captured_piece, ' ' | '\n') {
                        break;
                    }

                    // Add move for empty square
                    if captured_piece == '.' {
                        move_list.push(Move {
                            source: i,
                            target: target_square as usize,
                            piece,
                            captured_piece,
                        });
                    } else {
                        // Handle capture
                        if let Some(&captured_side) = self.colors.get(&captured_piece) {
                            if captured_side != piece_side && !matches!(captured_piece, 'K' | 'k') {
                                move_list.push(Move {
                                    source: i,
                                    target: target_square as usize,
                                    piece,
                                    captured_piece,
                                });
                            }
                        }
                        break; // Stop after hitting a piece (friendly or enemy)
                    }

                    // Stop for non-sliding pieces
                    if matches!(piece, 'N' | 'n' | 'K' | 'k') {
                        break;
                    }

                    // Continue sliding
                    target_square += offset;
                }
            }
        }

        move_list
    }

    fn make_move(&mut self, chess_move: &Move) {
        self.board[chess_move.target] = chess_move.piece;
        self.board[chess_move.source] = '.';
        if chess_move.piece == 'P' && self.rank_7.contains(&(chess_move.source as i32)) {
            self.board[chess_move.target] = 'Q';
        }
        if chess_move.piece == 'p' && self.rank_2.contains(&(chess_move.source as i32)) {
            self.board[chess_move.target] = 'q';
        }

        self.print_board();

        self.side = match self.side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    fn take_back(&mut self, chess_move: &Move) {
        self.board[chess_move.target] = chess_move.captured_piece;
        self.board[chess_move.source] = chess_move.piece;

        self.print_board();

        self.side = match self.side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chess = Chess::new("settings.json")?;
    let move_list: Vec<Move> = chess.generate_moves();

    for move_item in move_list.iter() {
        chess.make_move(move_item);
        chess.take_back(move_item);
    }

    Ok(())
}