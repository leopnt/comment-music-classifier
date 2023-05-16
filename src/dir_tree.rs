use crate::app_config::{AppConfig, Classification};
use std::fmt;
use trees::{Node, Tree};

pub struct DirTree {
    tree: Tree<String>,
}

#[derive(Debug)]
pub enum DirTreeError {
    CorruptedPath,
}

pub type DirTreeResult = Result<(), DirTreeError>;

impl DirTree {
    pub fn from_config(config: &AppConfig) -> Self {
        let mut reversed_classification = config.classification.clone();
        reversed_classification.reverse();

        let root = Tree::new(config.root_folder_name.clone());

        DirTree {
            tree: DirTree::tree_from_classification(reversed_classification, root),
        }
    }

    fn tree_from_classification(
        classification: Classification,
        mut parent: Tree<String>,
    ) -> Tree<String> {
        if classification.is_empty() {
            return parent;
        }

        let mut dir_floors = classification;

        for dir_floor in dir_floors.pop() {
            for dir in dir_floor {
                let child = Tree::new(dir);
                parent.push_back(DirTree::tree_from_classification(dir_floors.clone(), child));
            }
        }

        parent
    }

    pub fn put_into(&mut self, path: Vec<String>, filename: String) -> DirTreeResult {
        let mut path = path.clone();
        path.reverse();

        let target_node = DirTree::find_mut(&mut path, self.tree.root_mut().get_mut())
            .ok_or(DirTreeError::CorruptedPath);

        match target_node {
            Ok(v) => Ok(v.push_back(Tree::new(filename))),
            Err(e) => Err(e),
        }
    }

    fn find_mut<'a>(
        path: &mut Vec<String>,
        node: &'a mut Node<String>,
    ) -> Option<&'a mut Node<String>> {
        if path.is_empty() {
            Some(node)
        } else {
            let next_target_dir = path.pop().unwrap();

            let found = match node
                .iter_mut()
                .find(|child| *child.data() == next_target_dir)
            {
                Some(v) => v.get_mut(),
                None => return None,
            };

            DirTree::find_mut(path, found)
        }
    }
}

impl fmt::Display for DirTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tree.to_string())
    }
}
