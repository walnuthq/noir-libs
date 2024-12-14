use std::{env, path::PathBuf};

use toml_edit::{value, DocumentMut};

use crate::MANIFEST_FILE_NAME;

pub fn write_package(manifest : PathBuf, package_name: &str, path : &str) {
    println!("Adding value {}", path);
    let content = std::fs::read_to_string(manifest.clone()).expect("Cannot read file");
    let mut doc = content.parse::<DocumentMut>().expect("Invalid TOML");
    //doc["dependencies"][package_name] = value(path);

    let mut table = toml_edit::InlineTable::default();
    table.get_or_insert("path", path);
    // Assign to the dependencies table
    doc["dependencies"][package_name] = toml_edit::Item::Value(toml_edit::Value::InlineTable(table));

    std::fs::write(manifest, doc.to_string()).expect("Cannot write file");
}

pub fn try_find_manifest() -> Option<PathBuf> {
    let pwd = env::current_dir().expect("Unable to find current folder");
    //println!("DIR {:?}", pwd);
    let mut root = Some(pwd.as_path());
    while let Some(path) = root {
        let manifest = path.join(MANIFEST_FILE_NAME);
        if manifest.is_file() {
            return Some(manifest);
        }
        root = path.parent();
    }
    None
}