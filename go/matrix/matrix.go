package matrix

import (
	"errors"
	"strconv"
	"strings"
)

type Matrix struct {
	elems [][]int
	nrows int
	ncols int
}

func New(s string) (*Matrix, error) {
	m := &Matrix{}
	rows := strings.Split(s, "\n")
	m.nrows = len(rows)
	m.elems = make([][]int, m.nrows)

	for ix, row := range rows {
		row = strings.TrimSpace(row)
		cols := strings.Split(row, " ")
		for _, col := range cols {
			col = strings.TrimSpace(col)
			elem_int, err := strconv.Atoi(col)
			if err != nil {
				return nil, err
			}
			m.elems[ix] = append(m.elems[ix], elem_int)
		}
	}
	m.ncols = len(m.elems[0])

	if !m.IsValidMatrix() {
		return nil, errors.New("Invalid matrix")
	}

	return m, nil
}

func (m *Matrix) Cols() [][]int {
	var cols [][]int
	for ix := 0; ix < m.ncols; ix += 1 {
		var col []int
		for iy := 0; iy < m.nrows; iy += 1 {
			col = append(col, m.elems[iy][ix])
		}
		cols = append(cols, col)
	}
	return cols
}

func (m *Matrix) Rows() [][]int {
	var rows [][]int
	for ix := 0; ix < m.nrows; ix += 1 {
		var row []int
		for iy := 0; iy < m.ncols; iy += 1 {
			row = append(row, m.elems[ix][iy])
		}
		rows = append(rows, row)
	}
	return rows
}

func (m *Matrix) IsValidMatrix() bool {
	ncols := len(m.elems[0])
	for i := 1; i < m.nrows; i += 1 {
		if ncols != len(m.elems[i]) {
			return false
		}
	}
	return true
}

func (m *Matrix) Set(row, col, val int) bool {
	if (row < 0 || row >= m.nrows) ||
		(col < 0 || col >= m.ncols) {
		return false
	}
	m.elems[row][col] = val
	return true
}
