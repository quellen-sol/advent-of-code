package utils

import (
	"fmt"
	"iter"
)

// GridDirection is a step to a neighboring cell. y grows southwards.
type GridDirection int

const (
	North GridDirection = iota
	South
	East
	West
	NorthEast
	NorthWest
	SouthEast
	SouthWest
)

var (
	directionNames   = [...]string{"North", "South", "East", "West", "NorthEast", "NorthWest", "SouthEast", "SouthWest"}
	directionOffsets = [...][2]int{
		North:     {0, -1},
		South:     {0, 1},
		East:      {1, 0},
		West:      {-1, 0},
		NorthEast: {1, -1},
		NorthWest: {-1, -1},
		SouthEast: {1, 1},
		SouthWest: {-1, 1},
	}
	// Indexed by [dy+1][dx+1]. The middle is no direction, so it's never read.
	directionsByOffset = [3][3]GridDirection{
		{NorthWest, North, NorthEast},
		{West, -1, East},
		{SouthWest, South, SouthEast},
	}
	opposites  = [...]GridDirection{North: South, South: North, East: West, West: East, NorthEast: SouthWest, NorthWest: SouthEast, SouthEast: NorthWest, SouthWest: NorthEast}
	rightTurns = [...]GridDirection{North: East, East: South, South: West, West: North, NorthEast: SouthEast, SouthEast: SouthWest, SouthWest: NorthWest, NorthWest: NorthEast}
	leftTurns  = [...]GridDirection{North: West, West: South, South: East, East: North, NorthEast: NorthWest, NorthWest: SouthWest, SouthWest: SouthEast, SouthEast: NorthEast}

	// Clockwise from north.
	neighborDirections = []GridDirection{North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest}
	plusDirections     = []GridDirection{North, East, South, West}
	diagonalDirections = []GridDirection{NorthEast, SouthEast, SouthWest, NorthWest}
)

func (d GridDirection) Offset() (dx, dy int) {
	offset := directionOffsets[d]
	return offset[0], offset[1]
}

// GridDirectionFromOffset reports false when (dx, dy) isn't a step to a neighbor.
func GridDirectionFromOffset(dx, dy int) (GridDirection, bool) {
	if dx < -1 || dx > 1 || dy < -1 || dy > 1 || (dx == 0 && dy == 0) {
		return 0, false
	}
	return directionsByOffset[dy+1][dx+1], true
}

func (d GridDirection) Opposite() GridDirection {
	return opposites[d]
}

// RotateRight turns 90° clockwise.
func (d GridDirection) RotateRight() GridDirection {
	return rightTurns[d]
}

// RotateLeft turns 90° anticlockwise.
func (d GridDirection) RotateLeft() GridDirection {
	return leftTurns[d]
}

func (d GridDirection) String() string {
	if d < 0 || int(d) >= len(directionNames) {
		return fmt.Sprintf("GridDirection(%d)", int(d))
	}
	return directionNames[d]
}

// GridCreationItem is a value for GridFromIter to place, or a break that ends
// the current row. Make them with GridItem and GridBreak.
type GridCreationItem[T any] struct {
	Value T
	Break bool
}

func GridItem[T any](value T) GridCreationItem[T] {
	return GridCreationItem[T]{Value: value}
}

func GridBreak[T any]() GridCreationItem[T] {
	return GridCreationItem[T]{Break: true}
}

type Grid[T any] struct {
	InnerData [][]GridNode[T]
}

func NewGrid[T any](innerData [][]GridNode[T]) *Grid[T] {
	return &Grid[T]{InnerData: innerData}
}

// GridFromIter fills rows left to right, starting a new row at every break.
func GridFromIter[T any](items iter.Seq[GridCreationItem[T]]) *Grid[T] {
	var innerData [][]GridNode[T]
	var row []GridNode[T]
	for item := range items {
		if item.Break {
			innerData = append(innerData, row)
			row = nil
			continue
		}
		row = append(row, GridNode[T]{Value: item.Value, X: len(row), Y: len(innerData)})
	}

	if len(row) > 0 {
		innerData = append(innerData, row)
	}

	return &Grid[T]{InnerData: innerData}
}

// GetNode returns nil when (x, y) is outside the grid.
func (g *Grid[T]) GetNode(x, y int) *GridNode[T] {
	if y < 0 || y >= len(g.InnerData) || x < 0 || x >= len(g.InnerData[y]) {
		return nil
	}
	return &g.InnerData[y][x]
}

// Scan yields every node, row by row.
func (g *Grid[T]) Scan() iter.Seq[*GridNode[T]] {
	return func(yield func(*GridNode[T]) bool) {
		for y := range g.InnerData {
			row := g.InnerData[y]
			for x := range row {
				if !yield(&row[x]) {
					return
				}
			}
		}
	}
}

// IterRows yields every node, row by row.
func (g *Grid[T]) IterRows() iter.Seq[*GridNode[T]] {
	return g.Scan()
}

// IterRowsRev yields every node, row by row, starting from the bottom right.
func (g *Grid[T]) IterRowsRev() iter.Seq[*GridNode[T]] {
	return func(yield func(*GridNode[T]) bool) {
		for y := len(g.InnerData) - 1; y >= 0; y-- {
			row := g.InnerData[y]
			for x := len(row) - 1; x >= 0; x-- {
				if !yield(&row[x]) {
					return
				}
			}
		}
	}
}

// IterColumns yields every node, column by column.
func (g *Grid[T]) IterColumns() iter.Seq[*GridNode[T]] {
	return func(yield func(*GridNode[T]) bool) {
		for x := range g.NumColumns() {
			for y := range g.InnerData {
				if node := g.GetNode(x, y); node != nil && !yield(node) {
					return
				}
			}
		}
	}
}

// IterColumnsRev yields every node, column by column, starting from the bottom right.
func (g *Grid[T]) IterColumnsRev() iter.Seq[*GridNode[T]] {
	return func(yield func(*GridNode[T]) bool) {
		for x := g.NumColumns() - 1; x >= 0; x-- {
			for y := len(g.InnerData) - 1; y >= 0; y-- {
				if node := g.GetNode(x, y); node != nil && !yield(node) {
					return
				}
			}
		}
	}
}

// TraverseRow yields row y, west to east. It panics if the row doesn't exist.
func (g *Grid[T]) TraverseRow(y int) iter.Seq[*GridNode[T]] {
	if g.GetNode(0, y) == nil {
		panic(fmt.Sprintf("TraverseRow: row %d is outside the grid", y))
	}
	return g.IterBySlope(0, y, 0, 1)
}

// TraverseColumn yields column x, north to south. It panics if the column doesn't exist.
func (g *Grid[T]) TraverseColumn(x int) iter.Seq[*GridNode[T]] {
	if g.GetNode(x, 0) == nil {
		panic(fmt.Sprintf("TraverseColumn: column %d is outside the grid", x))
	}
	return g.IterBySlope(x, 0, 1, 0)
}

// IterBySlope steps by (rise, run) until it leaves the grid, starting with the
// node at (startX, startY).
func (g *Grid[T]) IterBySlope(startX, startY, rise, run int) iter.Seq[*GridNode[T]] {
	return func(yield func(*GridNode[T]) bool) {
		for x, y := startX, startY; ; x, y = x+run, y+rise {
			node := g.GetNode(x, y)
			if node == nil || !yield(node) {
				return
			}
		}
	}
}

func (g *Grid[T]) NumRows() int {
	return len(g.InnerData)
}

func (g *Grid[T]) NumColumns() int {
	if len(g.InnerData) == 0 {
		return 0
	}
	return len(g.InnerData[0])
}

type GridNode[T any] struct {
	Value T
	X, Y  int
}

// Equals compares positions only, whatever the nodes' values.
func (n *GridNode[T]) Equals(other *GridNode[T]) bool {
	return n.X == other.X && n.Y == other.Y
}

func (n *GridNode[T]) AddDirection(d GridDirection) (x, y int) {
	dx, dy := d.Offset()
	return n.X + dx, n.Y + dy
}

// GetNodeInDirection returns nil when the step leaves the grid.
func (n *GridNode[T]) GetNodeInDirection(g *Grid[T], d GridDirection) *GridNode[T] {
	return g.GetNode(n.AddDirection(d))
}

// Neighbors yields all eight surrounding nodes that exist, clockwise from north.
func (n *GridNode[T]) Neighbors(g *Grid[T]) iter.Seq2[GridDirection, *GridNode[T]] {
	return n.walk(g, neighborDirections)
}

// PlusWalk yields the orthogonal neighbors that exist, clockwise from north.
func (n *GridNode[T]) PlusWalk(g *Grid[T]) iter.Seq2[GridDirection, *GridNode[T]] {
	return n.walk(g, plusDirections)
}

// DiagonalWalk yields the diagonal neighbors that exist, clockwise from north-east.
func (n *GridNode[T]) DiagonalWalk(g *Grid[T]) iter.Seq2[GridDirection, *GridNode[T]] {
	return n.walk(g, diagonalDirections)
}

func (n *GridNode[T]) walk(g *Grid[T], directions []GridDirection) iter.Seq2[GridDirection, *GridNode[T]] {
	return func(yield func(GridDirection, *GridNode[T]) bool) {
		for _, d := range directions {
			if node := n.GetNodeInDirection(g, d); node != nil && !yield(d, node) {
				return
			}
		}
	}
}

// DirectionIter walks in a straight line to the edge of the grid. It starts
// with the next node in that direction, not with n.
func (n *GridNode[T]) DirectionIter(g *Grid[T], d GridDirection) iter.Seq[*GridNode[T]] {
	dx, dy := d.Offset()
	return g.IterBySlope(n.X+dx, n.Y+dy, dy, dx)
}

// TryGetDirection reports the direction of other, if it is one of n's neighbors.
func (n *GridNode[T]) TryGetDirection(other *GridNode[T]) (GridDirection, bool) {
	return GridDirectionFromOffset(other.X-n.X, other.Y-n.Y)
}

// Distance is the Manhattan distance.
func (n *GridNode[T]) Distance(other *GridNode[T]) int {
	return abs(other.Y-n.Y) + abs(other.X-n.X)
}

func (n *GridNode[T]) Slope(other *GridNode[T]) (rise, run int) {
	return other.Y - n.Y, other.X - n.X
}

// IterNodeBySlope steps by (rise, run) until it leaves the grid, starting with n.
func (n *GridNode[T]) IterNodeBySlope(g *Grid[T], rise, run int) iter.Seq[*GridNode[T]] {
	return g.IterBySlope(n.X, n.Y, rise, run)
}
