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
