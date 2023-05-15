use crate::app_config::{AppConfig, Classification};
use trees::Tree;
use std::fmt;

pub struct DirTree {
    tree: Tree<String>,
}

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
}

impl fmt::Display for DirTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.tree.to_string())
    }
}
