#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    EMPTY,
    CROSS,
    CIRCLE,
}

pub struct Game {
    board: [[Cell ; 3] ; 3]
}

impl Game {
    pub fn play(&mut self, x: usize, y: usize, player: Cell) -> Result<(), ()> {
        assert!(player == Cell::CIRCLE || player == Cell::CROSS);

        match self.board[y][x] {
            Cell::EMPTY => Ok(self.board[y][x] = player),
            _ => Err(())
        }
    }

    pub fn winning_position(&self) -> Cell {
        for i in 0..3 {
            // Horizontal alignment
            if self.board[0][i] == self.board[1][i] 
               && self.board[0][i] == self.board[2][i] 
               && self.board[2][i] != Cell::EMPTY {
                return self.board[0][i]
            }

            // Vertical alignment
            if self.board[i][0] == self.board[i][1] 
               && self.board[i][0] == self.board[i][2] 
               && self.board[i][2] != Cell::EMPTY {
                return self.board[i][0]
            }
        }

        // Diagonal \
        if self.board[0][0] == self.board[1][1] 
           && self.board[1][1] == self.board[2][2] 
           && self.board[1][1] != Cell::EMPTY {
            return self.board[1][1]
        }

        // Diagonal /
        if self.board[0][2] == self.board[1][1] 
           && self.board[1][1] == self.board[2][0] 
           && self.board[1][1] != Cell::EMPTY {
            return self.board[1][1]
        }

        Cell::EMPTY
    }

    pub fn new() -> Self {
        Game {
            board: [[Cell::EMPTY; 3]; 3]
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Displays the board nicely
        let mut string = String::from("  1   2   3\n");
        for y in 0..3 {
            string += &((y+1).to_string());
            string += " ";
            for x in 0..3 {
                match self.board[y][x] {
                    Cell::EMPTY => string += " ",
                    Cell::CROSS => string += "X",
                    Cell::CIRCLE => string += "O"
                }

                if x != 2 {
                    string += " | ";
                }
            }
            if y != 2 {
                string += "\n  ----------\n";
            }
        }
        write!(formatter, "{}", string)
    }
}