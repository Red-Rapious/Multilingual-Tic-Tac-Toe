#include <stdio.h>
#include <stdbool.h>

#define HUMAN 1
#define AI 2

void displayBoard(char board[3][3]) {
    printf("  1   2   3\n");
    for (int y = 0; y < 3; y++) {
        printf("%d ", y+1);

        for (int x = 0; x < 3; x++) {
            printf("%c", board[y][x]);

            if (x != 2) {
                printf(" | ");
            }
        }

        if (y != 2) {
            printf("\n  ----------\n");
        }
    }
}

char winningPosition(char board[3][3]) {
    for (int i = 0; i < 3; i++) {
        // Horizontal
        if (board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[1][i] != ' ') {
            return board[1][i];
        }

        // Vertical
        if (board[i][0] == board[i][1] && board[i][1] == board[i][2]  && board[i][1] != ' ') {
            return board[i][1];
        }
    }

    // Diagonals
    if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[1][1] != ' ') {
        return board[1][1];
    }

    if (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[1][1] != ' ') {
        return board[1][1];
    }
    
    // No winner
    return ' ';
}

int choosePlayerType(int playerNb) {
    int numberChosen = 0;
    while (numberChosen != HUMAN && numberChosen != AI) {
        printf("\nChoose a type for Player %d:\n(1: Human or 2: AI)\n>>> ", playerNb);
        scanf("%d", &numberChosen);
    }
    return numberChosen;
}

void askPosition(int playerNb, int* x, int* y, char board[3][3]) {
    printf("\n-- Player %d to play! --", playerNb);
    bool validPosition = false;

    while (!validPosition) {
        while (*x != 1 && *x != 2 && *x != 3) {
            printf("\nChoose a column to play (between 1 and 3):\n>>> ");
            scanf("%d", x);
        }

        while (*y != 1 && *y != 2 && *y != 3) {
            printf("\nChoose a lign to play (between 1 and 3):\n>>> ");
            scanf("%d", y);
        }

        if (board[*y-1][*x-1] == ' ') {
            validPosition = true;
        } else {
            printf("The cell (%d, %d) is occupied. Please choose an empty cell.", *x, *y);
            *x = 0;
            *y = 0;
        }
    }
}

int main()
{
    // Initialize an array of empty cells
    char board [3][3];
    for (int x = 0; x < 3; x++) {
        for (int y = 0; y < 3; y++) {
            board[x][y] = ' ';
        }
    }

    // Title
    printf("\n _____  ___   ___       _____  ___   ___       _____   ___   ___ \n|_   _||_ _| / __|     |_   _|/   \\ / __|     |_   _| / _ \\ | __|\n  | |   | | | (__        | |  | - || (__        | |  | (_) || _| \n  |_|  |___| \\___|       |_|  |_|_| \\___|       |_|   \\___/ |___|\n");

    // Player types
    int players [2] = {choosePlayerType(1), choosePlayerType(2)};
    
    char winning = ' ';
    int turn = 0;

    while (winning == ' ' && turn < 9) {
        printf("\n");
        displayBoard(board);
        printf("\n");

        int x = 0;
        int y = 0;

        switch (players[turn % 2])
        {
        case HUMAN:
            // Ask the player for the cell to choose
            askPosition(turn%2 + 1, &x, &y, board);

            // Determine the symbol to write in the cell
            char sign;
            if (turn % 2 == 0) {
                sign = 'X';
            } else {
                sign = 'O';
            }

            board[y-1][x-1] = sign;
            break;

        case AI:
            printf("Error: AI not implemented yet.");
            return 0;
        
        default:
            printf("Error: wrong player type");
            return 0;
        }

        winning = winningPosition(board);
        turn++;
    }

    if (winning == ' ') {
        printf("\n--- DRAW! ---\n");
    } else {
        printf("\n--- PLAYER %d WON! ---\n", turn%2 + 1);
    }
}