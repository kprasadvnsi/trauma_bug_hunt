mod data;
use std::{
    fs::{self, File},
    path::Path,
};

use data::{Context, Package};
use flate2::read::GzDecoder;
use tar::Archive;

mod downloader;
mod pkg_parser;
mod utils;

fn get_db_download_links(context: &Context, repo: String) -> Vec<String> {
    let mut db_download_list: Vec<String> = Vec::new();

    let repo_file_path = format!("{}/{}/{}/{}", context.mirror, context.arch, repo, repo);
    let repo_db = format!("{}.db", repo_file_path);
    let repo_db_tar_gz = format!("{}.db.tar.gz", repo_file_path);
    let repo_db_tar_gz_old = format!("{}.db.tar.gz.old", repo_file_path);
    let repo_files = format!("{}.files", repo_file_path);
    let repo_files_tar_gz = format!("{}.files.tar.gz", repo_file_path);
    let repo_files_tar_gz_old = format!("{}.files.tar.gz.old", repo_file_path);

    db_download_list.push(repo_db);
    db_download_list.push(repo_db_tar_gz);
    db_download_list.push(repo_db_tar_gz_old);
    db_download_list.push(repo_files);
    db_download_list.push(repo_files_tar_gz);
    db_download_list.push(repo_files_tar_gz_old);

    db_download_list
}

pub fn unpack_repo_db(repo_file_path: &str, repo_unpack_dest: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::open(repo_file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(repo_unpack_dest)?;

    Ok(())
}

pub fn get_packages(dir_path: &str) -> Result<Vec<Package>, std::io::Error> {
    let mut pkgs: Vec<Package> = Vec::new();
    for entry in fs::read_dir(Path::new(dir_path).to_owned())? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            let pkg_dir = path.file_name().unwrap().to_str().unwrap();
            let pkg_desc_path = format!("{}/{}/desc", dir_path, pkg_dir);
            let pkg = pkg_parser::parse_pkg_desc(pkg_desc_path);
            pkgs.push(pkg);
        }
    }

    Ok(pkgs)
}

fn get_remote_packages(context: &Context, repo: String) -> Vec<Package> {
    let temp_repo_db_path = format!("{}/{}/remote", context.tmp_dir, repo);
    utils::mkdir_if_not_exist(temp_repo_db_path.as_str());

    let db_dwn_list = get_db_download_links(context, repo.clone());

    println!("Downloading package database for repo: {}", repo);
    downloader::download(db_dwn_list, temp_repo_db_path.clone().as_str());
    let pkg_output_dir = utils::get_repo_cache_dir(context, repo.clone());
    utils::mkdir_if_not_exist(pkg_output_dir.as_str());

    utils::copy_files(temp_repo_db_path.as_str(), pkg_output_dir.as_str(), true);

    let remote_repo_db_dest = format!("{}/{}.db.tar.gz", temp_repo_db_path.as_str(), repo);

    unpack_repo_db(remote_repo_db_dest.as_str(), temp_repo_db_path.as_str()).unwrap();

    get_packages(temp_repo_db_path.as_str()).unwrap()
}

pub fn get_pkgs_download_link(
    context: &Context,
    repo: String,
    packages: Vec<Package>,
) -> Vec<String> {
    let mut pkg_download_list: Vec<String> = Vec::new();

    for package in packages {
        let pkg_download_link = format!(
            "{}/{}/{}/{}",
            context.mirror, context.arch, repo, package.filename
        );
        let pkg_sig_download_link = format!("{}.sig", pkg_download_link);

        pkg_download_list.push(pkg_download_link);
        pkg_download_list.push(pkg_sig_download_link);
    }

    pkg_download_list
}

fn clean_tmp_dir(context: &Context) {
    let tmp_path = Path::new(context.tmp_dir.as_str());
    if tmp_path.exists() {
        println!("Cleaning tmp_path");
        fs::remove_dir_all(tmp_path).unwrap();
    }
}

fn synchronize(context: &Context, repo: String) {
    clean_tmp_dir(context);
    let pkgs = get_remote_packages(context, repo.clone());

    let pkg_dwn_list = get_pkgs_download_link(context, repo.clone(), pkgs);

    let pkg_output_dir = utils::get_repo_cache_dir(context, repo.clone());
    println!("Downloading packages for repo: {}", repo);
    downloader::download(pkg_dwn_list, pkg_output_dir.as_str())
}

fn main() {
    println!("Trauma Bug Hunt");

    let context = Context {
        mirror: String::from("http://tw2.mirror.archlinuxarm.org"),
        arch: String::from("aarch64"),
        cache_dir: String::from("output"),
        tmp_dir: String::from("tmp"),
    };

    let repos = vec!["extra", "community"];

    for repo in repos {
        synchronize(&context, repo.to_string())
    }
}
