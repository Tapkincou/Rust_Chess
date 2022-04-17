pub mod board {

    #[derive(Debug)]
    pub struct Board {
        //better array because size is static
        pub current : Vec<Case> 

    }

    // Todo size problem
    #[derive(Debug)]
    pub struct Case(Position,Option<Piece>);


    impl Board {

        pub fn new() -> Self {

            let mut new_board : Vec<Case> = Vec::new();

            for l in 'a'..'h' {

                new_board.push(Case(Position(l,2),Some(Piece::new_paw(PieceColor::WHITE))));
                new_board.push(Case(Position(l,7),Some(Piece::new_paw(PieceColor::BLACK))));

                match l {
                    'a' => {
                        new_board.push(Case(Position(l,1),Some(Piece::new_rook(PieceColor::WHITE))));
                        new_board.push(Case(Position(l,8),Some(Piece::new_rook(PieceColor::BLACK))));
                    },
                    'b' => {},
                    'c' => {},
                    'd' => {},
                    'e' => {},
                    'f' => {},
                    'g' => {},
                    'h' => {},
                    _ => () // todo throw error


                }


            }


            Self{
                //TODO convert ?
                current : new_board
            }
            


            


        }


    }

    #[derive(Copy, Clone, Debug)]
    pub struct Position( char, i8 );

    #[derive(Debug)]
    pub enum PieceColor { BLACK, WHITE }

    #[derive(Debug)]
    pub enum PieceKind { PAW, ROOK, KING, QUEEN, KNIGHT, BISHOP }

    #[derive(Debug)]
    pub struct Piece {
        kind: PieceKind,
        color: PieceColor,
        possible_moves: Vec<Position> 
    }

    impl Piece {

        fn calculate_possible_moves(&self) -> Vec<Position> {


            self.get_move_pattern();
            
            vec![Position('a',1),Position('b',2),Position('c',3)]


        }

        fn get_move_pattern(&self) -> MovePattern {

            let move_pattern = match &self.kind {
                PieceKind::PAW      => get_paw_pattern(&self.color),
                PieceKind::BISHOP   => get_bishop_pattern(&self.color),
                PieceKind::ROOK     => get_rook_pattern(&self.color),
                PieceKind::QUEEN    => get_queen_pattern(&self.color),
                PieceKind::KING     => get_king_pattern(&self.color),
                PieceKind::KNIGHT   => get_knight_pattern(&self.color)
            };

            return move_pattern

        }

        // r# for escape reserved keywords and use them as identifiers
        pub fn r#move(&self, new_position: Position) -> Option<Position> {

            let possible_moves = self.calculate_possible_moves();

            for position in 0..possible_moves.len() {
                match new_position {
                    // default match 
                    _ => return Some(new_position)
                }
            }
            
            None

        }

        fn new_paw (color: PieceColor) -> Self {

            Self{
                color,
                kind : PieceKind::PAW,
                possible_moves : vec![]
            }


        } 
        fn new_rook (color: PieceColor) -> Self {

            Self{
                color,
                kind : PieceKind::ROOK,
                possible_moves : vec![]
            }


        } 

    }


    struct MovePattern {
        todo : Option<char>
    }

    fn get_paw_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }
    fn get_bishop_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }
    fn get_rook_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }
    fn get_queen_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }
    fn get_king_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }
    fn get_knight_pattern(color: &PieceColor) -> MovePattern { MovePattern { todo : None }  }

}

