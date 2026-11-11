use std::{
    cell::RefCell,
    fs::File,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use anyhow::Context;

use crate::{
    defs::Solution,
    utils::tree::{NodeRef, NodeRefable, Tree, TreeNode},
};

pub struct Day7Solution {
    input: String,
}

impl Solution<i64, i64> for Day7Solution {
    fn new(input_path: &str) -> Self {
        let input = Self::load_input(input_path);
        Self { input }
    }

    fn get_input(&self) -> &str {
        &self.input
    }

    fn get_part_1_solution(&self) -> i64 {
        self.make_tree()
            .dfs_iter()
            .filter_map(|n| {
                let val = &n.borrow().value;
                let size = val.get_size();

                if matches!(val, FileOrDir::Dir { .. }) && size <= 100000 {
                    return Some(size);
                }

                None
            })
            .sum()
    }

    fn get_part_2_solution(&self) -> i64 {
        let tree = self.make_tree();
        let used_space = tree.root.borrow().value.get_size();
        let free_space = 70000000 - used_space;
        let free_space_required = 30000000i64;

        tree.dfs_iter()
            .filter_map(|n| {
                let val = &n.borrow().value;
                let size = val.get_size();

                if matches!(val, FileOrDir::Dir { .. }) && size + free_space >= free_space_required
                {
                    return Some(size);
                }

                None
            })
            .min_by(|a, b| a.cmp(b))
            .unwrap()
    }
}

impl Day7Solution {
    fn make_tree(&self) -> Tree<FileOrDir> {
        let mut root_node = NodeRef::new(RefCell::new(TreeNode::new(FileOrDir::Dir {
            name: "/".into(),
            size: 0,
        })));
        let mut current_node_ref = NodeRef::clone(&root_node);

        for line in self.get_lines().map(Self::parse_line) {
            match line {
                ParsedLine::Command(Command::ChangeDir(dir)) => match dir {
                    ".." => {
                        let parent_copy = { current_node_ref.borrow().parent.clone().unwrap() };
                        current_node_ref = parent_copy;
                    }
                    "/" => {
                        current_node_ref = root_node.clone();
                    }
                    other => {
                        let new_ref = {
                            current_node_ref
                                .borrow()
                                .children
                                .iter()
                                .find(|c| match &c.borrow().deref().value {
                                    FileOrDir::Dir { name, .. } => name == other,
                                    FileOrDir::File { name, .. } => false,
                                })
                                .cloned()
                                .context(format!(
                                    "Could not find dir {other} in {}",
                                    current_node_ref.borrow().deref().value.get_name()
                                ))
                                .unwrap()
                        };
                        current_node_ref = new_ref;
                    }
                },
                ParsedLine::Command(Command::List) => {
                    // tbh do nothing bc we just want the output following lines
                }
                ParsedLine::Output(file_or_dir) => {
                    let name = file_or_dir.get_name();
                    let child_exists = current_node_ref
                        .borrow()
                        .children
                        .iter()
                        .any(|c| c.borrow().deref().value.get_name() == name);

                    if child_exists {
                        continue;
                    }

                    if let FileOrDir::File {
                        name,
                        size: filesize,
                    } = &file_or_dir
                    {
                        let mut size_calc_ref = Some(current_node_ref.clone());
                        while let Some(parent_dir) = size_calc_ref.clone() {
                            let mut parent_mut = parent_dir.borrow_mut();
                            let FileOrDir::Dir {
                                name,
                                size: dirsize,
                            } = &mut parent_mut.deref_mut().value
                            else {
                                unreachable!(
                                    "Files should not have children, but {} does?",
                                    parent_mut.value.get_name()
                                );
                            };

                            *dirsize += filesize;

                            size_calc_ref = parent_mut.parent.clone();
                        }
                    }
                    current_node_ref.insert_child(file_or_dir);
                }
            }
        }

        Tree::new_from_ref(root_node)
    }

    fn parse_line(line: &str) -> ParsedLine {
        let mut s = line.split(' ');
        let mut curr = s.next();
        if curr.is_some_and(|v| v == "$") {
            // is command
            curr = s.next();
            match curr {
                Some("ls") => ParsedLine::Command(Command::List),
                Some("cd") => {
                    curr = s.next();
                    ParsedLine::Command(Command::ChangeDir(curr.unwrap()))
                }
                Some(_) | None => {
                    unreachable!("Bad input to command in input?");
                }
            }
        } else {
            // Output of ls
            match curr {
                Some("dir") => {
                    curr = s.next();
                    ParsedLine::Output(FileOrDir::Dir {
                        name: curr.unwrap().into(),
                        size: 0,
                    })
                }
                Some(file_size) => {
                    curr = s.next();
                    ParsedLine::Output(FileOrDir::File {
                        name: curr.unwrap().into(),
                        size: file_size.parse().unwrap(),
                    })
                }
                None => {
                    unreachable!("Should be matched to a command if no actual output...");
                }
            }
        }
    }
}

enum ParsedLine<'a> {
    Command(Command<'a>),
    Output(FileOrDir),
}

enum Command<'a> {
    List,
    ChangeDir(&'a str),
}

enum FileOrDir {
    File { name: String, size: i64 },
    Dir { name: String, size: i64 },
}

impl FileOrDir {
    pub fn get_name(&self) -> &str {
        match self {
            FileOrDir::Dir { name, .. } => name,
            FileOrDir::File { name, .. } => name,
        }
    }
    pub fn get_size(&self) -> i64 {
        match self {
            FileOrDir::Dir { size, .. } => *size,
            FileOrDir::File { size, .. } => *size,
        }
    }
}
