use std::env;

use crate::manifest::remove_package;

pub fn remove(package_name: &str) {
    let pwd = env::current_dir().expect("Unable to find current folder");
    remove_package(pwd, package_name);
}
