package utils

import (
	"slices"
	"strings"
	"testing"
)

func TestVowels(t *testing.T) {
	if got := strings.Join(Vowels, ""); got != "aeiou" {
		t.Errorf("got %q", got)
	}
}

func TestGCD(t *testing.T) {
	if got := GCD(12, -18); got != 6 {
		t.Errorf("GCD(12, -18) = %d, want 6", got)
	}
	if got := GCD(0, 5); got != 5 {
		t.Errorf("GCD(0, 5) = %d, want 5", got)
	}
	if got := GCD[uint](12, 18); got != 6 {
		t.Errorf("GCD[uint](12, 18) = %d, want 6", got)
	}
}

func TestLCM(t *testing.T) {
	cases := []struct{ a, b, want int64 }{{4, 6, 12}, {-4, 6, 12}, {0, 5, 0}}
	for _, c := range cases {
		if got := LCM(c.a, c.b); got != c.want {
			t.Errorf("LCM(%d, %d) = %d, want %d", c.a, c.b, got, c.want)
		}
	}
}

func TestStraightDistance(t *testing.T) {
	if got := (Point2D{0, 0}).StraightDistance(Point2D{3, -4}); got != 5 {
		t.Errorf("got %v, want 5", got)
	}
	if got := (Point3D{1, 2, 3}).StraightDistance(Point3D{3, 5, 9}); got != 7 {
		t.Errorf("got %v, want 7", got)
	}
}

func TestGridlikeCornerRectArea(t *testing.T) {
	cases := []struct {
		a, b Point2D
		want int64
	}{
		{Point2D{1, 1}, Point2D{3, 4}, 12},
		{Point2D{0, 5}, Point2D{5, 0}, 36},
		{Point2D{5, 0}, Point2D{0, 5}, 36},
	}
	for _, c := range cases {
		if got := c.a.GridlikeCornerRectArea(c.b); got != c.want {
			t.Errorf("%v to %v: got %d, want %d", c.a, c.b, got, c.want)
		}
	}
}

func TestChunk1(t *testing.T) {
	expectChunks(t, "ABCDEFG", 2, []string{"AB", "CD", "EF", "G"})
}

func TestChunk2(t *testing.T) {
	expectChunks(t, "ABCDEFG", 3, []string{"ABC", "DEF", "G"})
}

func TestChunksCountCharacters(t *testing.T) {
	expectChunks(t, "αβγ", 2, []string{"αβ", "γ"})
}

func TestEmptyStringHasNoChunks(t *testing.T) {
	expectChunks(t, "", 2, nil)
}

func expectChunks(t *testing.T, s string, size int, expected []string) {
	t.Helper()
	if got := slices.Collect(StringChunks(s, size)); !slices.Equal(got, expected) {
		t.Errorf("got %q, want %q", got, expected)
	}
}

func TestTreeDFSIter(t *testing.T) {
	root := NewTreeNode("/")
	a := root.InsertChild("a")
	root.InsertChild("b")
	c := a.InsertChild("c")
	tree := NewTree(root)

	var order []string
	for node := range tree.DFSIter() {
		order = append(order, node.Value)
	}
	if expected := []string{"/", "b", "a", "c"}; !slices.Equal(order, expected) {
		t.Errorf("got %v, want %v", order, expected)
	}
	if c.Parent != a || a.Parent != root {
		t.Error("children should point at their parents")
	}
}

func TestTreeRootHasNoParent(t *testing.T) {
	a := NewTreeNode("/").InsertChild("a")
	if NewTree(a).Root.Parent != nil {
		t.Error("the root shouldn't have a parent")
	}
}
