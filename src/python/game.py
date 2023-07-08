from enum import Enum

class Cell(Enum):
    EMPTY = 0
    CROSS = 1
    CIRCLE = 2

class CellOccupiedError(Exception):
    # This exception is raised when an occupied cell is selected by a player
    pass

class Game:
    board = [[Cell.EMPTY for _ in range(3)] for _ in range(3)]

    def play(self, x: int, y: int, player: Cell) -> None:
        assert(player == Cell.CIRCLE or player == Cell.CROSS)

        match self.board[y][x]:
            case Cell.EMPTY:
                self.board[y][x] = player
            case _:
                raise CellOccupiedError
            
    def winning_position(self) -> Cell:
        for i in range(3):
            # Horizontal alignment
            if self.board[0][i] == self.board[1][i] == self.board[2][i] != Cell.EMPTY:
                return self.board[0][i]
            
            # Vertical alignment
            if self.board[i][0] == self.board[i][1] == self.board[i][2] != Cell.EMPTY:
                return self.board[i][0]
            
        if self.board[0][0] == self.board[1][1] == self.board[2][2] != Cell.EMPTY:
                return self.board[1][1]
        if self.board[0][2] == self.board[1][1] == self.board[2][0] != Cell.EMPTY:
                return self.board[1][1]
        
        return Cell.EMPTY


    def __str__(self):
        # Displays the board nicely
        string = "  1   2   3\n"
        for y in range(3):
            string += str(y+1) + " "
            for x in range(3):
                match self.board[y][x]:
                    case Cell.EMPTY:
                        string += " "
                    case Cell.CROSS:
                        string += "X"
                    case Cell.CIRCLE:
                        string += "O"
                if x != 2:
                    string += " | "
            if y != 2:
                string += "\n"
                string += "  " + "-"*10
                string += "\n"
        return string