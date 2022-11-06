use fs_extra::dir::{copy, CopyOptions};
use std::{fs, path::Path};

use crate::data::Context;

pub fn get_repo_cache_dir(context: &Context, repo: String) -> String {
    format!("{}/{}/{}", context.cache_dir, context.arch, repo)
}

pub fn copy_files(source: &str, destination: &str, overwrite: bool) {
    let mut options = CopyOptions::new();
    options.content_only = true;

    if overwrite {
        options.overwrite = true;
    }

    copy(source, destination, &options).unwrap();
}

pub fn mkdir_if_not_exist(dir_source: &str) {
    let dir_path = Path::new(dir_source);
    if !dir_path.exists() {
        //println!("{} not exist", dir_source);
        fs::create_dir_all(dir_path).unwrap();
    } else {
        //println!("{} already exist", dir_source);
    }
}
