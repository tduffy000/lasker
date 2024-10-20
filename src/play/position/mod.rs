use super::{
    board::Board,
    error::FENParsingError,
    move_gen,
    r#move::MoveList,
    types::{CastlingRights, Color, PieceType, Square},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Position {
    pub board: Board,
    pub side_to_move: Color,
    pub castling_permission_history: Vec<CastlingRights>, // bits = [ wK, wQ, bK, bQ ] 
    pub en_passant_history: Vec<Option<Square>>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            board: Board::default(),
            side_to_move: Color::White,
            castling_permission_history: vec![CastlingRights::all()],
            en_passant_history: vec![None],
        }
    }
}

impl Position {
    pub fn from_fields(fields: Vec<String>) -> Result<Position, FENParsingError> {
        let n_req_fields = 4;
        if fields.len() != n_req_fields {
            return Err(FENParsingError::new(format!(
                "Incorrect number of fields to parse position. Expected {}, got {}",
                n_req_fields,
                fields.len()
            )));
        }

        let board = Board::from_fen(&fields[0])?;
        let side_to_move = if fields[1] == "b".to_string() {
            Color::Black
        } else {
            Color::White
        };
        let castling_permission_history = vec![CastlingRights::from_fen(&fields[2])?];
        let en_passant_history = vec![Square::from_fen(&fields[3])?];

        let pos = Position { board, side_to_move, castling_permission_history, en_passant_history };
        Ok(pos)
    }

    ///
    ///
    pub fn from_fen(fen: impl ToString) -> Result<Position, FENParsingError> {
        let fields: Vec<String> = fen.to_string().split(' ').map(|s| s.to_string()).collect();
        Position::from_fields(fields)
    }

    pub fn flip_side(&mut self) {
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn legal_moves(&self) -> MoveList {
        let color = self.side_to_move;
        let mut moves = MoveList::empty();

        for piece in self.board.pieces(color) {
            for sq in self.board.bitboard(piece) {
                if piece.piece_type() == PieceType::Pawn {
                    move_gen::generate_pawn_moves(&self, sq, &mut moves);
                } else {
                    move_gen::generate_moves(&self, piece, sq, &mut moves);
                }
            }
        }
        moves
    }

    pub fn castling_permissions(&self) -> CastlingRights {
        *self.castling_permission_history.last().unwrap()
    }

    pub fn en_passant(&self) -> Option<Square> {
        *self.en_passant_history.last().unwrap()
    }

    // TODO: use piece Type here
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_fields() {
        let ok_fields: Vec<String> = vec!["8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8", "b", "-", "-"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(Position::from_fields(ok_fields).is_ok());
        let err_fields: Vec<String> = vec!["8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8", "b"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert!(Position::from_fields(err_fields).is_err());
    }

    #[test]
    fn test_from_fen() {
        let ok_fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - -";
        assert!(Position::from_fen(ok_fen).is_ok());
        let err_fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b";
        assert!(Position::from_fen(err_fen).is_err());
    }

    #[test]
    fn test_legal_moves() {}
}
