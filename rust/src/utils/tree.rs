use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

pub struct Tree<T> {
    pub root: NodeRef<T>,
}

impl<T> Tree<T> {
    pub fn new_from_root(mut node: TreeNode<T>) -> Self {
        node.parent = None;
        Self {
            root: NodeRef::new(RefCell::new(node)),
        }
    }

    pub fn new_from_ref(node: NodeRef<T>) -> Self {
        node.borrow_mut().parent = None;
        Self { root: node }
    }

    pub fn dfs_iter(&self) -> TreeDFSIter<T> {
        TreeDFSIter {
            stack: vec![self.root.clone()],
            visited: HashSet::new(),
        }
    }

    pub fn bfs_iter(&self) -> TreeBFSIter<T> {
        let mut vq = VecDeque::new();
        vq.push_back(self.root.clone());
        TreeBFSIter {
            queue: vq,
            visited: HashSet::new(),
        }
    }
}

pub struct TreeDFSIter<T> {
    stack: Vec<NodeRef<T>>,
    visited: HashSet<NodeRef<T>>,
}

impl<T> Iterator for TreeDFSIter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        for child in node.borrow().children.iter() {
            self.stack.push(child.clone());
        }

        Some(node)
    }
}

pub struct TreeBFSIter<T> {
    queue: VecDeque<NodeRef<T>>,
    visited: HashSet<NodeRef<T>>,
}

impl<T> Iterator for TreeBFSIter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        for child in node.borrow().children.iter() {
            self.queue.push_back(child.clone());
        }

        Some(node)
    }
}

pub type NodeRef<T> = Rc<RefCell<TreeNode<T>>>;

pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<NodeRef<T>>,
    pub parent: Option<NodeRef<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
            parent: None,
        }
    }
}

pub trait NodeRefable<T> {
    fn insert_child(&self, value: T) -> NodeRef<T>;
}

impl<T> NodeRefable<T> for NodeRef<T> {
    fn insert_child(&self, value: T) -> NodeRef<T> {
        let mut self_mut = self.borrow_mut();
        let mut new_node = TreeNode::new(value);
        new_node.parent = Some(self.clone());
        let new_node = NodeRef::new(RefCell::new(new_node));
        self_mut.children.push(new_node.clone());

        new_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `/` has children `a` and `b`; `a` has `c`, and `b` has `d`.
    fn make_tree() -> Tree<&'static str> {
        let tree = Tree::new_from_root(TreeNode::new("/"));
        tree.root.insert_child("a").insert_child("c");
        tree.root.insert_child("b").insert_child("d");
        tree
    }

    fn values(nodes: impl Iterator<Item = NodeRef<&'static str>>) -> Vec<&'static str> {
        nodes.map(|node| node.borrow().value).collect()
    }

    #[test]
    fn test_dfs_iter() {
        let tree = make_tree();
        assert_eq!(values(tree.dfs_iter()), ["/", "b", "d", "a", "c"]);
        assert_eq!(values(tree.dfs_iter().take(2)), ["/", "b"]);
    }

    #[test]
    fn test_bfs_iter() {
        let tree = make_tree();
        assert_eq!(values(tree.bfs_iter()), ["/", "a", "b", "c", "d"]);
        assert_eq!(values(tree.bfs_iter().take(2)), ["/", "a"]);
    }

    #[test]
    fn test_children_point_at_their_parents() {
        let tree = make_tree();
        let a = tree.root.borrow().children[0].clone();
        let c = a.borrow().children[0].clone();
        assert!(Rc::ptr_eq(a.borrow().parent.as_ref().unwrap(), &tree.root));
        assert!(Rc::ptr_eq(c.borrow().parent.as_ref().unwrap(), &a));
    }
}
