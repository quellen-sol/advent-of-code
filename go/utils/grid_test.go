package utils

import (
	"iter"
	"slices"
	"strings"
	"testing"
)

const testGrid = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`

func makeGrid() *Grid[rune] {
	return GridFromIter(func(yield func(GridCreationItem[rune]) bool) {
		for _, r := range testGrid {
			item := GridItem(r)
			if r == '\n' {
				item = GridBreak[rune]()
			}
			if !yield(item) {
				return
			}
		}
	})
}

func values(nodes iter.Seq[*GridNode[rune]]) string {
	var b strings.Builder
	for node := range nodes {
		b.WriteRune(node.Value)
	}
	return b.String()
}

type step struct {
	direction GridDirection
	value     rune
}

func steps(walk iter.Seq2[GridDirection, *GridNode[rune]]) []step {
	var got []step
	for d, node := range walk {
		got = append(got, step{d, node.Value})
	}
	return got
}

func expectValues(t *testing.T, nodes iter.Seq[*GridNode[rune]], expected string) {
	t.Helper()
	if got := values(nodes); got != expected {
		t.Errorf("got %q, want %q", got, expected)
	}
}

func expectPrefix(t *testing.T, nodes iter.Seq[*GridNode[rune]], expected string) {
	t.Helper()
	if got := values(nodes); !strings.HasPrefix(got, expected) {
		t.Errorf("got %q, want it to start with %q", got, expected)
	}
}

func expectSteps(t *testing.T, walk iter.Seq2[GridDirection, *GridNode[rune]], expected []step) {
	t.Helper()
	if got := steps(walk); !slices.Equal(got, expected) {
		t.Errorf("got %v, want %v", got, expected)
	}
}

func TestGridFromIter(t *testing.T) {
	grid := makeGrid()
	if got := grid.GetNode(0, 9).Value; got != 'M' {
		t.Errorf("got %q, want 'M'", got)
	}
	if grid.NumRows() != 10 || grid.NumColumns() != 10 {
		t.Errorf("got %dx%d, want 10x10", grid.NumColumns(), grid.NumRows())
	}
}

func TestGridGetNode(t *testing.T) {
	grid := makeGrid()
	if got := grid.GetNode(0, 0).Value; got != 'M' {
		t.Errorf("got %q, want 'M'", got)
	}
	if got := grid.GetNode(1, 1).Value; got != 'S' {
		t.Errorf("got %q, want 'S'", got)
	}
}

func TestGridGetNodeOutsideTheGrid(t *testing.T) {
	grid := makeGrid()
	for _, p := range [][2]int{{-1, 0}, {0, -1}, {10, 0}, {0, 10}} {
		if node := grid.GetNode(p[0], p[1]); node != nil {
			t.Errorf("GetNode(%d, %d) = %+v, want nil", p[0], p[1], node)
		}
	}
}

func TestGridNeighbors(t *testing.T) {
	grid := makeGrid()
	expectSteps(t, grid.GetNode(0, 0).Neighbors(grid), []step{{East, 'M'}, {SouthEast, 'S'}, {South, 'M'}})
}

func TestDirectionalIter(t *testing.T) {
	grid := makeGrid()
	expectPrefix(t, grid.GetNode(0, 0).DirectionIter(grid, South), "MAMX")
}

func TestGridIterColumns(t *testing.T) {
	expectPrefix(t, makeGrid().IterColumns(), "MMAM")
}

func TestGridIterColumnsRev(t *testing.T) {
	expectPrefix(t, makeGrid().IterColumnsRev(), "XMAS")
}

func TestGridIterRows(t *testing.T) {
	expectValues(t, makeGrid().IterRows(), strings.ReplaceAll(testGrid, "\n", ""))
}

func TestGridIterRowsRev(t *testing.T) {
	expectPrefix(t, makeGrid().IterRowsRev(), "XSAM")
}

func TestGridPlusWalk(t *testing.T) {
	grid := makeGrid()
	expectSteps(t, grid.GetNode(1, 1).PlusWalk(grid), []step{{North, 'M'}, {East, 'A'}, {South, 'M'}, {West, 'M'}})
}

func TestGridPlusWalk2(t *testing.T) {
	grid := makeGrid()
	expectSteps(t, grid.GetNode(0, 0).PlusWalk(grid), []step{{East, 'M'}, {South, 'M'}})
}

func TestDiagonalWalkOnAnEdge(t *testing.T) {
	grid := makeGrid()
	expectSteps(t, grid.GetNode(9, 1).DiagonalWalk(grid), []step{{SouthWest, 'M'}, {NorthWest, 'S'}})
}

func TestTraverseRowAndColumnIncludeTheFirstNode(t *testing.T) {
	grid := makeGrid()
	expectValues(t, grid.TraverseRow(0), "MMMSXXMASM")
	expectValues(t, grid.TraverseColumn(0), "MMAMXXSSMM")
}

func TestIterBySlope(t *testing.T) {
	grid := makeGrid()
	expectValues(t, grid.IterBySlope(0, 0, 1, 1), "MSXMAXSAMX")
	expectValues(t, grid.GetNode(0, 9).IterNodeBySlope(grid, -1, 2), "MMMSM")
}

func TestIteratorsStopEarly(t *testing.T) {
	grid := makeGrid()
	for node := range grid.IterColumns() {
		if node.Value != 'M' {
			t.Errorf("got %q, want 'M'", node.Value)
		}
		break
	}
}

func TestNodeRelations(t *testing.T) {
	grid := makeGrid()
	origin := grid.GetNode(0, 0)

	if d, ok := origin.TryGetDirection(grid.GetNode(1, 1)); !ok || d != SouthEast {
		t.Errorf("got %v %v, want SouthEast", d, ok)
	}
	if _, ok := origin.TryGetDirection(grid.GetNode(2, 2)); ok {
		t.Error("a node two steps away isn't a neighbor")
	}
	if _, ok := origin.TryGetDirection(origin); ok {
		t.Error("a node isn't its own neighbor")
	}
	if got := origin.Distance(grid.GetNode(3, 4)); got != 7 {
		t.Errorf("got distance %d, want 7", got)
	}
	if rise, run := origin.Slope(grid.GetNode(3, 4)); rise != 4 || run != 3 {
		t.Errorf("got slope (%d, %d), want (4, 3)", rise, run)
	}
	if x, y := origin.AddDirection(NorthWest); x != -1 || y != -1 {
		t.Errorf("got (%d, %d), want (-1, -1)", x, y)
	}
}

func TestNodesAreEqualByPosition(t *testing.T) {
	a := &GridNode[rune]{Value: 'a', X: 1, Y: 2}
	if !a.Equals(&GridNode[rune]{Value: 'b', X: 1, Y: 2}) {
		t.Error("nodes at the same position should be equal")
	}
	if a.Equals(&GridNode[rune]{Value: 'a', X: 2, Y: 1}) {
		t.Error("nodes at different positions shouldn't be equal")
	}
}

func TestDirectionRotations(t *testing.T) {
	cases := []struct{ got, want GridDirection }{
		{North.RotateRight(), East},
		{NorthWest.RotateRight(), NorthEast},
		{North.RotateLeft(), West},
		{SouthEast.RotateLeft(), NorthEast},
	}
	for _, c := range cases {
		if c.got != c.want {
			t.Errorf("got %v, want %v", c.got, c.want)
		}
	}

	for _, d := range neighborDirections {
		if got := d.RotateRight().RotateLeft(); got != d {
			t.Errorf("%v right then left is %v", d, got)
		}
		if got := d.RotateRight().RotateRight(); got != d.Opposite() {
			t.Errorf("%v turned around is %v, but its opposite is %v", d, got, d.Opposite())
		}
	}
}

func TestDirectionOffsets(t *testing.T) {
	if dx, dy := SouthWest.Offset(); dx != -1 || dy != 1 {
		t.Errorf("got (%d, %d), want (-1, 1)", dx, dy)
	}
	if d, ok := GridDirectionFromOffset(1, -1); !ok || d != NorthEast {
		t.Errorf("got %v %v, want NorthEast", d, ok)
	}
	for _, offset := range [][2]int{{0, 0}, {2, 0}} {
		if d, ok := GridDirectionFromOffset(offset[0], offset[1]); ok {
			t.Errorf("%v isn't a direction, got %v", offset, d)
		}
	}
	for _, d := range neighborDirections {
		if got, _ := GridDirectionFromOffset(d.Offset()); got != d {
			t.Errorf("%v's offset maps back to %v", d, got)
		}
	}
}
