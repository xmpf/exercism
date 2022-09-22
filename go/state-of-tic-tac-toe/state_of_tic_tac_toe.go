package stateoftictactoe

import (
	"errors"
)

type State string

const (
	Win     State = "win"
	Ongoing State = "ongoing"
	Draw    State = "draw"
)

func IsValidBoard(board []string) (bool, []int) {

	var xs int = 0
	var os int = 0
	var marks int = 0

	for _, row := range board {
		for _, mark := range row {
			switch mark {
			case 'X':
				xs += 1
			case 'O':
				os += 1
			default:
				continue
			}
			marks += 1
		}
	}

	player_x := IsWinningBoard(board, 'X')
	player_o := IsWinningBoard(board, 'O')
	ret := []int{marks, xs, os}

	if (xs == os+1) || (xs == os) {
		switch {
		case player_o && player_x:
			return false, ret
		case player_o:
			return xs == os, ret
		case player_x && xs != os+1:
			return false, ret
		default:
			return true, ret
		}
	}
	return false, ret
}

func IsWinningBoard(board []string, mark byte) bool {

	// check horizontals
	for row := 0; row < 3; row += 1 {
		if board[row][0] == board[row][1] &&
			board[row][0] == board[row][2] &&
			board[row][0] == mark {
			return true
		}
	}

	// check verticals
	for col := 0; col < 3; col += 1 {
		if board[0][col] == board[1][col] &&
			board[0][col] == board[2][col] &&
			board[0][col] == mark {
			return true
		}
	}

	// check diagonal #1
	if board[0][0] == board[1][1] &&
		board[0][0] == board[2][2] &&
		board[0][0] == mark {
		return true
	}

	// check diagonal #2
	if board[0][2] == board[1][1] &&
		board[0][2] == board[2][0] &&
		board[0][2] == mark {
		return true
	}

	return false
}

func GameOver(board []string) bool {
	return IsWinningBoard(board, 'X') ||
		IsWinningBoard(board, 'O')
}

func StateOfTicTacToe(board []string) (State, error) {
	valid, res := IsValidBoard(board)
	if !valid {
		return "", errors.New("Invalid board")
	}

	// Win
	if GameOver(board) {
		return Win, nil
	}

	// Draw
	if res[0] == 9 {
		return Draw, nil
	}

	// Loose
	return Ongoing, nil
}
