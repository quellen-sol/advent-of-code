package utils

import "iter"

type TreeNode[T any] struct {
	Value    T
	Children []*TreeNode[T]
	Parent   *TreeNode[T]
}

func NewTreeNode[T any](value T) *TreeNode[T] {
	return &TreeNode[T]{Value: value}
}

// InsertChild adds a new node holding value as n's last child, and returns it.
func (n *TreeNode[T]) InsertChild(value T) *TreeNode[T] {
	child := &TreeNode[T]{Value: value, Parent: n}
	n.Children = append(n.Children, child)
	return child
}

type Tree[T any] struct {
	Root *TreeNode[T]
}

func NewTree[T any](root *TreeNode[T]) *Tree[T] {
	root.Parent = nil
	return &Tree[T]{Root: root}
}

// DFSIter visits every node before its children. Siblings come out last to first.
func (t *Tree[T]) DFSIter() iter.Seq[*TreeNode[T]] {
	return func(yield func(*TreeNode[T]) bool) {
		stack := []*TreeNode[T]{t.Root}
		for len(stack) > 0 {
			node := stack[len(stack)-1]
			stack = append(stack[:len(stack)-1], node.Children...)
			if !yield(node) {
				return
			}
		}
	}
}
