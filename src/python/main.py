from game import Game, Cell, CellOccupiedError
from enum import Enum

class PlayerType(Enum):
    HUMAN = 1
    AI = 2

def choose_player_type(player_number: int) -> PlayerType:
    number_chosen = 0
    while number_chosen != 1 and number_chosen != 2:
        try:
            number_chosen = int(input("Choose a type for Player " + str(player_number) + ":\n(1: Human or 2: AI)\n>>> "))
        except (TypeError, ValueError, EOFError):
            print("Please enter a number.\n")

        if number_chosen != 1 and number_chosen != 2:
            print("Please enter a number that is 1 (Human) or 2 (AI).\n")
    return PlayerType.HUMAN if 1 else PlayerType.AI

def ask_position(player_number: int) -> tuple:
    print("-- Player", player_number, "to play! --")
    x = 0
    while x not in [1, 2, 3]:
        try:
            x = int(input("Choose a column to play (between 1 and 3):\n>>> "))
        except (TypeError, ValueError, EOFError):
            print("Please enter a number.\n")

        if x not in [1, 2, 3]:
            print("Please enter a number that is between 1 and 3.\n")

    y = 0
    while y not in [1, 2, 3]:
        try:
            y = int(input("Choose a lign to play (between 1 and 3):\n>>> "))
        except (TypeError, ValueError, EOFError):
            print("Please enter a number.\n")

        if y not in [1, 2, 3]:
            print("Please enter a number that is between 1 and 3.\n")

    return x-1, y-1


if __name__ == "__main__":
    tictactoe = Game()

    print("""
 _____  ___   ___       _____  ___   ___       _____   ___   ___ 
|_   _||_ _| / __|     |_   _|/   \ / __|     |_   _| / _ \ | __|
  | |   | | | (__        | |  | - || (__        | |  | (_) || _| 
  |_|  |___| \___|       |_|  |_|_| \___|       |_|   \___/ |___|""")

    players = (choose_player_type(1), choose_player_type(2))
    winning = Cell.EMPTY
    turn = 0

    while winning == Cell.EMPTY and turn < 9:
        print("")
        print(tictactoe)
        print("")

        match players[turn % 2]:
            case PlayerType.HUMAN:
                correct_position = False
                while not correct_position:
                    try:
                        x, y = ask_position(turn + 1)
                        tictactoe.play(x, y, Cell.CROSS if turn % 2 == 0 else Cell.CIRCLE)
                        correct_position = True
                    except CellOccupiedError:
                        print("The cell ", (x, y), "is occupied. Please choose an empty cell.")

            case PlayerType.AI:
                print("Not implemented yet.")
                exit()

        winning = tictactoe.winning_position()
        turn += 1

    print(tictactoe)
    if turn == 9:
        print("\n--- DRAW! ---")
    else:
        print("\n--- PLAYER", turn%2 + 1, "WON! ---")