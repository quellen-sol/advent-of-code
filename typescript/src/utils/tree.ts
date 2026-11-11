export class TreeNode<T> {
    children: TreeNode<T>[] = [];
    parent: TreeNode<T> | undefined = undefined;

    constructor(public value: T) {}

    /** Adds a new node holding `value` as this node's last child, and returns it. */
    insertChild(value: T): TreeNode<T> {
        const child = new TreeNode(value);
        child.parent = this;
        this.children.push(child);
        return child;
    }
}

export class Tree<T> {
    constructor(public root: TreeNode<T>) {
        root.parent = undefined;
    }

    /** Visits every node before its children. Siblings come out last to first. */
    *dfsIter(): Generator<TreeNode<T>> {
        const stack = [this.root];
        let node: TreeNode<T> | undefined;
        while ((node = stack.pop()) !== undefined) {
            for (const child of node.children) {
                stack.push(child);
            }
            yield node;
        }
    }

    /** Visits the nodes level by level. Siblings come out first to last. */
    *bfsIter(): Generator<TreeNode<T>> {
        const queue = [this.root];
        for (let i = 0; i < queue.length; i++) {
            const node = queue[i]!;
            for (const child of node.children) {
                queue.push(child);
            }
            yield node;
        }
    }
}
