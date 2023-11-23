#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceColor{
    None,
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType{
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoardPiece{
    piece: PieceType,
    color: PieceColor,
}

pub fn make_blank_board() -> [[BoardPiece; 8]; 8] {
    [[BoardPiece{ piece: PieceType::None, color: PieceColor::None}; 8]; 8]
}

pub struct GameState {
    pub board: [[BoardPiece; 8]; 8],
}

impl GameState {
    pub fn start_position(&mut self) {
        //row of white pieces
        self.board[0][0] = BoardPiece{piece: PieceType::Rook, color: PieceColor::White};
        self.board[1][0] = BoardPiece{piece: PieceType::Knight, color: PieceColor::White};
        self.board[2][0] = BoardPiece{piece: PieceType::Bishop, color: PieceColor::White};
        self.board[3][0] = BoardPiece{piece: PieceType::Queen, color: PieceColor::White};
        self.board[4][0] = BoardPiece{piece: PieceType::King, color: PieceColor::White};
        self.board[5][0] = BoardPiece{piece: PieceType::Bishop, color: PieceColor::White};
        self.board[6][0] = BoardPiece{piece: PieceType::Knight, color: PieceColor::White};
        self.board[7][0] = BoardPiece{piece: PieceType::Rook, color: PieceColor::White};
        //row of white pawns
        self.board[0][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[1][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[2][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[3][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[4][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[5][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[6][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        self.board[7][1] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::White};
        //row of black pawns
        self.board[0][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[1][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[2][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[3][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[4][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[5][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[6][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        self.board[7][6] = BoardPiece{piece: PieceType::Pawn, color: PieceColor::Black};
        //row of black pieces
        self.board[0][7] = BoardPiece{piece: PieceType::Rook, color: PieceColor::Black};
        self.board[1][7] = BoardPiece{piece: PieceType::Knight, color: PieceColor::Black};
        self.board[2][7] = BoardPiece{piece: PieceType::Bishop, color: PieceColor::Black};
        self.board[3][7] = BoardPiece{piece: PieceType::Queen, color: PieceColor::Black};
        self.board[4][7] = BoardPiece{piece: PieceType::King, color: PieceColor::Black};
        self.board[5][7] = BoardPiece{piece: PieceType::Bishop, color: PieceColor::Black};
        self.board[6][7] = BoardPiece{piece: PieceType::Knight, color: PieceColor::Black};
        self.board[7][7] = BoardPiece{piece: PieceType::Rook, color: PieceColor::Black};
    }

    pub fn print_board(&self){
        let mut label: String;
        
        let white_pawn = BoardPiece {piece: PieceType::Pawn, color: PieceColor::White};
        let white_knight = BoardPiece {piece: PieceType::Knight, color: PieceColor::White};
        let white_bishop = BoardPiece {piece: PieceType::Bishop, color: PieceColor::White};
        let white_rook = BoardPiece {piece: PieceType::Rook, color: PieceColor::White};
        let white_queen = BoardPiece {piece: PieceType::Queen, color: PieceColor::White};
        let white_king = BoardPiece {piece: PieceType::King, color: PieceColor::White};

        let black_pawn = BoardPiece {piece: PieceType::Pawn, color: PieceColor::Black};
        let black_knight = BoardPiece {piece: PieceType::Knight, color: PieceColor::Black};
        let black_bishop = BoardPiece {piece: PieceType::Bishop, color: PieceColor::Black};
        let black_rook = BoardPiece {piece: PieceType::Rook, color: PieceColor::Black};
        let black_queen = BoardPiece {piece: PieceType::Queen, color: PieceColor::Black};
        let black_king = BoardPiece {piece: PieceType::King, color: PieceColor::Black};

        for row in 0..8{
            for col in 0..8{
                if self.board[col][row] == white_pawn {
                    label = "P".to_string();
                } else if self.board[col][row] == white_knight {
                    label = "N".to_string();
                } else if self.board[col][row] == white_bishop {
                    label = "B".to_string();
                } else if self.board[col][row] == white_rook {
                    label = "R".to_string();
                } else if self.board[col][row] == white_queen {
                    label = "Q".to_string();
                } else if self.board[col][row] == white_king {
                    label = "K".to_string();
                } else if self.board[col][row] == black_pawn {
                    label = "p".to_string();
                } else if self.board[col][row] == black_knight {
                    label = "n".to_string();
                } else if self.board[col][row] == black_bishop {
                    label = "b".to_string();
                } else if self.board[col][row] == black_rook {
                    label = "r".to_string();
                } else if self.board[col][row] == black_queen {
                    label = "q".to_string();
                } else if self.board[col][row] == black_king {
                    label = "k".to_string();
                } else {
                    label = "-".to_string();
                }
                print!("{}", label);
            }
            println!();
        }
        println!();
    }
}