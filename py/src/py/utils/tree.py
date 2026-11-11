from collections.abc import Iterator


class TreeNode[T]:
    __slots__ = ("value", "children", "parent")

    def __init__(self, value: T) -> None:
        self.value = value
        self.children: list[TreeNode[T]] = []
        self.parent: TreeNode[T] | None = None

    def insert_child(self, value: T) -> TreeNode[T]:
        """Adds a new node holding value as this node's last child, and returns it."""
        child = TreeNode(value)
        child.parent = self
        self.children.append(child)
        return child


class Tree[T]:
    __slots__ = ("root",)

    def __init__(self, root: TreeNode[T]) -> None:
        root.parent = None
        self.root = root

    def dfs_iter(self) -> Iterator[TreeNode[T]]:
        """Visits every node before its children. Siblings come out last to first."""
        stack = [self.root]
        while stack:
            node = stack.pop()
            stack.extend(node.children)
            yield node
