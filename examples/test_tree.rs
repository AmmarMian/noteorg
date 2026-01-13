use noteorg::traversal::CategoryTree;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let path = Path::new("tests/dir_structure_example/");
    let categories = CategoryTree::try_from(path)?;
    categories.display();

    Ok(())
}
