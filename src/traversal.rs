// Traversal related utilities
use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CategoryTree {
    pub name: String,
    pub children: Vec<CategoryTree>,
}

impl CategoryTree {
    pub fn display(&self) {
        self.display_helper("", true, true);
    }

    fn display_helper(&self, prefix: &str, is_last: bool, is_root: bool) {
        // Print current node
        if is_root {
            println!("{}/", self.name);
        } else {
            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, self.name);
        }

        // Print children
        let child_count = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == child_count - 1;
            let extension = if is_root {
                String::new()
            } else if is_last {
                "    ".to_string()
            } else {
                "│   ".to_string()
            };
            let new_prefix = format!("{}{}", prefix, extension);
            child.display_helper(&new_prefix, is_last_child, false);
        }
    }

    pub fn get_categories(&self, path: &Path) -> io::Result<Vec<String>> {
        // Get from path to the root of this category tree
        let path_match = if path.is_file() {
            path.parent().unwrap()
        } else {
            path
        };
        let path_iter = path_match
            .iter()
            .skip_while(|&x| x.to_str().unwrap() != self.name)
            .skip(1); // Skip 1 because
        // we can't look for root in its children.
        let mut result: Vec<String> = vec![];
        let mut current_node = self;

        for category_name in path_iter {
            match current_node
                .children
                .iter()
                .find(|&x| category_name.to_str().unwrap() == x.name)
            {
                Some(subcategory) => {
                    result.push(category_name.to_string_lossy().into_owned());
                    current_node = subcategory;
                }
                None => {
                    return Err(io::Error::other(
                        "The path given does not match the category tree",
                    ));
                }
            }
        }
        Ok(result)
    }
}

impl TryFrom<&Path> for CategoryTree {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        parse_categories_tree_from_path(path)
    }
}

pub fn parse_categories_tree_from_path(path: &Path) -> io::Result<CategoryTree> {
    let name = path.file_name().unwrap().to_string_lossy().to_string();

    let children = fs::read_dir(path)?
        .filter_map(|entry| entry.ok()) // We
        // discard errors
        .filter(|entry| entry.path().is_dir())
        .map(|entry| parse_categories_tree_from_path(&entry.path()))
        .collect::<io::Result<Vec<_>>>()?;

    Ok(CategoryTree { name, children })
}

pub fn show_dir_contents(path: &Path, recursion_level: Option<usize>) -> io::Result<()> {
    let level = recursion_level.unwrap_or(0);
    let filename = path.file_name().unwrap().display();
    if path.is_dir() {
        println!("{}📁 {}", "  ".repeat(level), filename);
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let sub_path = entry.path();
            show_dir_contents(sub_path.as_path(), Some(level + 1))?;
        }
    } else {
        println!("{}📄 {}", "  ".repeat(level), filename);
    }
    Ok(())
}

pub fn get_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    let result = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .flat_map(|entry| get_files(entry.path().as_path()).unwrap_or_default())
        .collect();
    Ok(result)
}
